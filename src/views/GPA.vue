<template>
  <h1>绩点计算器</h1>
  <div>
    <div v-if="grades.length == 0">
      <textarea
        placeholder="请在这里输入你的cookie"
        class="InputCookie"
        v-model="cookie"
      ></textarea>
      <button @click="fetchGrades2">确认</button>
    </div>
    <div class="filter-section" v-if="grades.length > 0">
      <div>
        <button @click="resetData">重置数据</button>
        <button @click="resetCookie">重新设置cookie</button>
      </div>
      <div class="filter-group">
        <h3>学期筛选：</h3>
        <div class="checkbox-group">
          <label v-for="term in termOptions" :key="term">
            <input type="checkbox" v-model="selectedTerms" :value="term" />
            {{ term }}
          </label>
        </div>
      </div>

      <div class="filter-group">
        <h3>课程类型：</h3>
        <div class="checkbox-group">
          <label v-for="type in typeOptions" :key="type">
            <input type="checkbox" v-model="selectedTypes" :value="type" />
            {{ type }}
          </label>
        </div>
      </div>
    </div>
    <div class="table-container" v-if="grades.length > 0">
      <table>
        <thead>
          <tr>
            <th>选择</th>
            <th>学期</th>
            <th>课程名称</th>
            <th>课程类型</th>
            <th>学分</th>
            <th>成绩</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(grade, index) in filteredGrades" :key="index">
            <td>
              <input
                type="checkbox"
                v-model="grade.selected"
                @change="updateSelection"
              />
            </td>
            <td>{{ grade.XNXQDM_DISPLAY }}</td>
            <td>{{ grade.XSKCM }}</td>
            <td>{{ grade.KCXZDM_DISPLAY }}</td>
            <td>{{ grade.XF }}</td>
            <td>{{ grade.XSZCJMC }}</td>
          </tr>
        </tbody>
      </table>
    </div>
    <div v-if="grades.length > 0" class="gpa-display">
      <h3>平均绩点：{{ gpaInfo.gpa }}</h3>
      <h3>总学分数：{{ gpaInfo.finalCredits }}</h3>
    </div>
  </div>
</template>

<script setup>
import { invoke } from "@tauri-apps/api/core";
import { ref, computed, watch, onMounted } from "vue";
import {
  exists,
  readTextFile,
  remove,
  writeTextFile,
  mkdir
} from "@tauri-apps/plugin-fs";
import { join, appDataDir } from '@tauri-apps/api/path';
import * as path from "@tauri-apps/api/path";

const cookie = ref("");
const grades = ref([]);

async function getStoragePath() {
  const isDev = await invoke("is_dev");
  if (isDev) {
    return await appDataDir();
  } else {
    return await appDataDir();
  }
}

onMounted(async () => {
  try {
    const basePath = await getStoragePath();
    const filePath = await path.join(basePath, "gpa-data", "grades.json");

    if (await exists(filePath)) {
      const contents = await readTextFile(filePath);
      try {
        const parsed = JSON.parse(contents);
        if (Array.isArray(parsed) && parsed.length > 0) {
          grades.value = parsed;
        }
      } catch (e) {
        console.error("本地数据损坏，已忽略");
        await remove(filePath);
      }
    }
  } catch (error) {
    console.error("本地缓存加载失败:", error);
  }
});
// 获取成绩
async function fetchGrades2() {
  try {
    const response = await invoke("fetch_grades2", {
      cookie: cookie.value,
      url: "https://ehall.seu.edu.cn/jwapp/sys/cjcx/modules/cjcx/xscjcx.do",
    });

    // 数据预处理
    const rawData = JSON.parse(response);

    // 验证数据格式
    if (!Array.isArray(rawData)) {
      throw new Error("接口返回数据格式异常，预期为数组");
    }

    // 数据清洗和格式化
    grades.value = rawData
      .map((grade) => {
        // 必填字段验证
        const requiredFields = [
          "XNXQDM_DISPLAY",
          "XSKCM",
          "KCXZDM_DISPLAY",
          "XSZCJMC",
          "XF",
        ];
        if (requiredFields.some((field) => !(field in grade))) {
          console.warn("异常数据项：", grade);
          return null;
        }

        // 学分格式化
        const formatCredit = (xf) => {
          if (typeof xf === "string") {
            // 处理 .5 格式
            if (xf.startsWith(".")) return parseFloat(`0${xf}`);
            // 处理全角字符
            return parseFloat(xf.replace(/[^0-9.]/g, ""));
          }
          return parseFloat(xf);
        };

        return {
          ...grade,
          // 强制转换类型
          XNXQDM_DISPLAY: grade.XNXQDM_DISPLAY.toString().trim(),
          XSKCM: grade.XSKCM.toString().trim(),
          KCXZDM_DISPLAY: grade.KCXZDM_DISPLAY.toString().trim(),
          XSZCJMC: grade.XSZCJMC.toString().trim(),
          XF: formatCredit(grade.XF),
          selected: true,
          // 添加原始数据备份
          _raw: grade,
        };
      })
      .filter(Boolean); // 过滤无效数据

    console.log("结构化成绩数据:", grades.value);

    // 数据统计
    const courseCount = grades.value.length;
    const creditSum = grades.value.reduce((sum, g) => sum + g.XF, 0);
    console.log(
      `成功加载 ${courseCount} 门课程，总学分 ${creditSum.toFixed(1)}`
    );

    await saveGradesToLocal(grades.value);
  } catch (error) {
    console.error("请求失败:", error);
    // 可视化错误提示
    alert(`数据加载失败：${error.message}`);
    grades.value = []; // 清空数据防止显示异常
  }
}

// 保存数据到本地
async function saveGradesToLocal(data) {
  try {
    const basePath = await getStoragePath(); // 新增: 获取存储路径
    const filePath = await path.join(basePath, "gpa-data", "grades.json"); // 修改: 构建完整路径

    // 确保路径中的所有目录都已存在
    const dirPath = await path.dirname(filePath);
    await mkdir(dirPath, { recursive: true }); // 新增: 创建目录

    // ✅ 在 `writeTextFile` 里传 `{ dir: path.BaseDirectory.AppData }`
    await writeTextFile(filePath, JSON.stringify(data), {
      create: true,
    });

    console.log("成绩数据已保存");
  } catch (error) {
    console.error("本地保存失败:", error);
  }
}

// 重置数据
function resetData() {
  selectedTerms.value = [...termOptions.value];
  selectedTypes.value = ["必修", "限选"];
  grades.value.forEach((grade) => {
    grade.selected = true;
  });
}

// 清除本地数据
async function resetCookie() {
  try {
    const basePath = await getStoragePath(); // 新增: 获取存储路径
    const filePath = await path.join(basePath, "gpa-data", "grades.json"); // 修改: 构建完整路径

    if (await exists(filePath)) {
      await remove(filePath);
    }
  } catch (error) {
    console.error("清除本地数据失败:", error);
  }
  grades.value = [];
}

// 新增的响应式数据
const selectedTerms = ref([]);
const selectedTypes = ref(["必修", "限选"]);

// 获取所有学期选项（需在获取数据后执行）
const termOptions = computed(() => {
  return [...new Set(grades.value.map((g) => g.XNXQDM_DISPLAY))];
});

// 获取所有课程类型选项
const typeOptions = computed(() => {
  return [...new Set(grades.value.map((g) => g.KCXZDM_DISPLAY))];
});

// 筛选后的成绩数据
const filteredGrades = computed(() => {
  return grades.value.filter((grade) => {
    const termMatch =
      selectedTerms.value.length === 0 ||
      selectedTerms.value.includes(grade.XNXQDM_DISPLAY);
    const typeMatch = selectedTypes.value.includes(grade.KCXZDM_DISPLAY);
    return termMatch && typeMatch;
  });
});

// 在获取数据后初始化选中项
watch(grades, (newVal) => {
  if (newVal.length > 0) {
    selectedTerms.value = [...termOptions.value];
  }
});

const selectedFilteredGrades = computed(() =>
  filteredGrades.value.filter((grade) => grade.selected)
);

// 添加绩点计算函数
const gpaInfo = computed(() => {
  let totalPoints = 0;
  let totalCredits = 0;

  selectedFilteredGrades.value.forEach((grade) => {
    // 成绩处理逻辑
    const score = grade.XSZCJMC; // 假设成绩是数值类型
    const credit = parseFloat(grade.XF); // 学分
    let point = 0; //绩点

    // 等级制成绩判断
    if (isNaN(score)) {
      switch (score.trim()) {
        case "优":
          point = 4.5;
          break;
        case "良":
          point = 3.5;
          break;
        case "中":
          point = 2.5;
          break;
        case "及格":
          point = 1.5;
          break;
        case "不及格":
          point = 0;
          break;
        default:
          return; // 忽略无法识别的等级
      }
    } else {
      // 数值型成绩转换
      const numericScore = parseFloat(score);
      if (numericScore >= 96) point = 4.8;
      else if (numericScore >= 93) point = 4.5;
      else if (numericScore >= 90) point = 4.0;
      else if (numericScore >= 86) point = 3.8;
      else if (numericScore >= 83) point = 3.5;
      else if (numericScore >= 80) point = 3.0;
      else if (numericScore >= 76) point = 2.8;
      else if (numericScore >= 73) point = 2.5;
      else if (numericScore >= 70) point = 2.0;
      else if (numericScore >= 66) point = 1.8;
      else if (numericScore >= 63) point = 1.5;
      else if (numericScore >= 60) point = 1.0;
      else point = 0;
    }

    if (!isNaN(credit)) {
      totalPoints += credit * point;
      totalCredits += credit;
    }
  });

  // 处理除零错误并保留两位小数
  return {
    gpa: totalCredits > 0 ? (totalPoints / totalCredits).toFixed(4) : "0.0000",
    finalCredits: totalCredits,
  };
});
</script>

<style>
.InputCookie {
  height: 100px;
  width: 500px;
  margin-right: 50px;
  white-space: normal;
  overflow-wrap: break-word;
  overflow-y: auto;
  resize: none;
}

.table-container {
  max-height: 60vh;
  overflow-y: auto;
  border: 1px solid #ddd;
  border-radius: 8px;
  margin: 16px 0;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

table {
  width: 100%;
  border-collapse: collapse;
  background: white;
}

thead {
  position: sticky;
  top: 0;
  background: #f8f9fa;
  z-index: 1;
  box-shadow: 0 2px 2px -1px rgba(0, 0, 0, 0.1);
}

th,
td {
  padding: 12px 16px;
  text-align: left;
  border-bottom: 1px solid #eee;
}

th {
  background: #f8f9fa;
  font-weight: 500;
}

tr:hover {
  background-color: #f5f5f5;
}
</style>