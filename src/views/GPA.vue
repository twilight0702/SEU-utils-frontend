<template>
  <!-- 顶部栏 -->
  <div class="header-container">
    <div class="head">绩点计算器</div>
    <router-link to="/" class="back-link"> 返回主页 </router-link>
  </div>

  <div>
    <!-- cookie输入部分 -->
    <div v-if="grades.length == 0">
      <textarea
        placeholder="请在这里输入你的cookie"
        class="InputCookie"
        v-model="cookie"
      ></textarea>
      <button @click="fetchGrades2" class="check-button">确认</button>
      <div class="cookie-tip" @click="showTutorial = true">
        不知道如何获取cookie?
      </div>
    </div>

    <!-- 重置操作按钮栏 -->
    <div class="filter-section" v-if="grades.length > 0">
      <div>
        <!-- 鼠标悬停提示语 -->
        <div
          v-if="tooltip.visible"
          class="tooltip"
          :style="{ left: `${tooltip.x}px`, top: `${tooltip.y}px` }"
        >
          {{ tooltip.text }}
        </div>

        <button
          @mouseenter.mouse="showTooltip('重置筛选条件和选中状态', $event)"
          @mousemove="updatePosition"
          @mouseleave="hideTooltip"
          @click="resetData"
          class="reset-button"
        >
          重置筛选条件
        </button>
        <button
          @mouseenter.mouse="
            showTooltip(
              '重新输入cookie爬取数据，注意会删除本地缓存数据',
              $event
            )
          "
          @mousemove="updatePosition"
          @mouseleave="hideTooltip"
          @click="resetCookie"
          class="reset-button"
        >
          重新设置cookie
        </button>
      </div>

      <!--  筛选条件栏 -->
      <div class="filter-group">
        <h3 style="font-weight: bold">学期筛选：</h3>
        <div class="checkbox-group">
          <label v-for="term in termOptions" :key="term">
            <input type="checkbox" v-model="selectedTerms" :value="term" />
            {{ term === "2023-2024学年2学期" ? "2023-2024学年秋季学期" : term }}
          </label>
        </div>
      </div>

      <div class="filter-group">
        <h3 style="font-weight: bold">课程类型：</h3>
        <div class="checkbox-group">
          <label v-for="type in typeOptions" :key="type">
            <input type="checkbox" v-model="selectedTypes" :value="type" />
            {{ type }}
          </label>
        </div>
      </div>
    </div>

    <!-- 课程信息显示栏 -->
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
            <td>
              {{
                grade.XNXQDM_DISPLAY === "2023-2024学年2学期"
                  ? "2023-2024学年秋季学期"
                  : grade.XNXQDM_DISPLAY
              }}
            </td>
            <td>{{ grade.XSKCM }}</td>
            <td>{{ grade.KCXZDM_DISPLAY }}</td>
            <td>{{ grade.XF }}</td>
            <td>{{ grade.XSZCJMC }}</td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- 绩点计算结果 -->
    <div v-if="grades.length > 0" class="gpa-display">
      <h3 style="font-weight: bold">平均绩点：{{ gpaInfo.gpa }}</h3>
      <h3 style="font-weight: bold">总学分数：{{ gpaInfo.finalCredits }}</h3>
    </div>

    <!-- cookie获取教程提示框和遮罩 -->
    <transition name="modal">
      <div
        v-if="showTutorial"
        class="modal-mask"
        @click.self="showTutorial = false"
      >
        <transition name="modal-container">
          <div class="modal-container">
            <div class="modal-header">
              <h2>如何获取Cookie？</h2>
              <button @click="showTutorial = false" class="modal-close">
                &times;
              </button>
            </div>
            <div class="modal-content">
              <ol>
                <li>
                  登录<a
                    href="https://ehall.seu.edu.cn/new/index.html"
                    target="_blank"
                    >东南大学网上办事服务大厅</a
                  >，进入成绩查询页面，按F12打开开发者工具
                </li>
                <li>切换到Network（网络）标签页</li>
                <li>刷新页面，找到任意以.do结尾的请求</li>
                <li>在Request Headers中找到Cookie字段</li>
                <li>
                  复制整个Cookie值粘贴到输入框
                </li>
              </ol>
            </div>
          </div>
        </transition>
      </div>
    </transition>
  </div>
</template>

<script setup>
import { invoke } from "@tauri-apps/api/core";
import { ref, computed, watch, onMounted, reactive } from "vue";
import {
  exists,
  readTextFile,
  remove,
  writeTextFile,
  mkdir,
} from "@tauri-apps/plugin-fs";
import { join, appDataDir } from "@tauri-apps/api/path";
import * as path from "@tauri-apps/api/path";

const cookie = ref("");
const grades = ref([]);

//统一设置存储基路径（现开发版和构建版都使用本地C:\Users\用户名\AppData\Roaming\com.seu.utils.frontend文件夹）
async function getStoragePath() {
  const isDev = await invoke("is_dev");
  if (isDev) {
    return await appDataDir();
  } else {
    return await appDataDir();
  }
}

// 页面生成时从本地缓存加载数据
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
  if (cookie.value.trim() == "") {
    alert("请输入cookie!");
    return;
  }
  try {
    const response = await invoke("fetch_grades2", {
      cookie: cookie.value.trim(),
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
    const basePath = await getStoragePath();
    const filePath = await path.join(basePath, "gpa-data", "grades.json");

    // 确保路径中的所有目录都已存在
    const dirPath = await path.dirname(filePath);
    await mkdir(dirPath, { recursive: true });

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

// 清除本地数据，重新输入cookie
async function resetCookie() {
  try {
    const basePath = await getStoragePath();
    const filePath = await path.join(basePath, "gpa-data", "grades.json");

    if (await exists(filePath)) {
      await remove(filePath);
    }
  } catch (error) {
    console.error("清除本地数据失败:", error);
  }
  grades.value = [];
}

// 存储筛选选项
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

// 根据选择筛选后的成绩数据
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

// 计算选中的筛选后的成绩数据
const selectedFilteredGrades = computed(() =>
  filteredGrades.value.filter((grade) => grade.selected)
);

// 绩点计算函数
const gpaInfo = computed(() => {
  let totalPoints = 0;
  let totalCredits = 0;

  selectedFilteredGrades.value.forEach((grade) => {
    // 成绩处理逻辑
    const score = grade.XSZCJMC; 
    const credit = parseFloat(grade.XF); // 学分
    let point = 0; //当前这门课的绩点

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

  // 处理除零错误并保留四位小数
  return {
    gpa: totalCredits > 0 ? (totalPoints / totalCredits).toFixed(4) : "0.0000",
    finalCredits: totalCredits,
  };
});

// 新增tooltip相关逻辑（鼠标悬浮显示提示文字）
const tooltip = reactive({
  visible: false,
  text: "",
  x: 0,
  y: 0,
  timeoutId: null,
});

// 更新鼠标位置
const updatePosition = (e) => {
  tooltip.x = e.clientX + 15;
  tooltip.y = e.clientY + 15;
};

// 显示提示（带300ms延迟）
const showTooltip = (text, e) => {
  tooltip.timeoutId = setTimeout(() => {
    tooltip.visible = true;
    updatePosition(e);
    tooltip.text = text;
  }, 300);
};

// 隐藏提示
const hideTooltip = () => {
  clearTimeout(tooltip.timeoutId);
  tooltip.visible = false;
};

// 用于判断是否显示cookie获取教程
const showTutorial = ref(false);
</script>

<style scoped>
/* 新增容器样式 */
.header-container {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 10px;
}

/* 保持原有标题样式 */
.head {
  font-size: 40px;
  margin: 0; /* 移除默认margin */
  font-weight: bold;
}

/* 新增返回按钮样式 */
.back-link {
  padding: 8px 16px;
  background: #69a197;
  color: white;
  border-radius: 4px;
  text-decoration: none;
  transition: background 0.3s;
}

.back-link:hover {
  background: #e5c94c;
  color: #ffffff;
}

.check-button {
  font-size: 15px;
  width: 100px;
  padding: 5px;
  margin-right: 20px;
  background-color: #69a197;
  color: white;
  transition: background 0.3s;
  border-radius: 4px;
  border: none;
  position: relative;
  top: -50px; /* 向上移动10像素，数值可调 */
}

.check-button:hover {
  background: #e5c94c;
}

.reset-button {
  font-size: 15px;
  min-width: 50px;
  padding: 5px;
  margin-right: 20px;
  color: #69a197;
  transition: background 0.3s;
  border-radius: 4px;
  border: none;
}

.reset-button:hover {
  background: #eae2be;
}

.InputCookie {
  height: 100px;
  width: 500px;
  margin-right: 50px;
  white-space: normal;
  overflow-wrap: break-word;
  overflow-y: auto;
  resize: none;
  border-radius: 5px;
}

.filter-group {
  margin: 10px;
}

/* 调整复选框组容器 */
.checkbox-group {
  display: flex;
  flex-wrap: wrap;
  gap: 12px; /* 元素间统一间距 */
  margin: 8px 0; /* 上下留白 */
}

/* 调整每个选项标签 */
.checkbox-group label {
  display: inline-flex;
  align-items: center;
  margin: 4px 8px; /* 上下4px 左右8px */
  padding: 6px 12px; /* 内边距 */
  background: #f5f5f5;
  border-radius: 4px;
  transition: background 0.2s;
}

/* 悬停效果 */
.checkbox-group label:hover {
  background: #eaeaea;
}

/* 复选框间距 */
.checkbox-group input[type="checkbox"] {
  margin-right: 6px; /* 复选框与文字间距 */
  transform: scale(1.1);
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

/* 添加tooltip样式 */
.tooltip {
  position: fixed;
  background: rgba(0, 0, 0, 0.8);
  color: white;
  padding: 8px 12px;
  border-radius: 4px;
  font-size: 14px;
  pointer-events: none;
  z-index: 1000;
  white-space: nowrap;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
  animation: tooltipFade 0.2s ease-out;
}

@keyframes tooltipFade {
  from {
    opacity: 0;
    transform: translateY(-5px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/*以下是指导获取cookie的教程的样式 */
.modal-mask {
  /* 原有样式保持不变 */
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 9999;
}

.modal-container {
  /* 原有样式保持不变 */
  background: white;
  border-radius: 8px;
  width: 600px;
  max-width: 90%;
  max-height: 80vh;
  overflow-y: auto;
  padding: 20px;
  position: relative;
}

/* 修改蒙层动画 */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}
.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

/* 修改容器动画 */
/* 修改动画样式 */
.modal-enter-active .modal-container,
.modal-leave-active .modal-container {
  transition: all 0.3s cubic-bezier(0.68, -0.55, 0.27, 1.55);
}

.modal-enter-from .modal-container,
.modal-leave-to .modal-container {
  transform: scale(0.8) translateY(20px);
  opacity: 0;
}

.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

/* 添加尺寸变化的过渡属性 */
.modal-container {
  transition: transform 0.3s ease, opacity 0.3s ease, max-height 0.3s ease;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.modal-close {
  font-size: 24px;
  background: none;
  border: none;
  cursor: pointer;
  color: #666;
}

.modal-content {
  line-height: 1.6;
}

.cookie-tip {
  color: #69a197;
  cursor: pointer;
  margin: 10px 0;
  text-decoration: underline;
}

.cookie-tip:hover {
  color: #4d8077;
}

.demo-image {
  width: 100%;
  margin-top: 15px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

ol {
  padding-left: 20px;
}

li {
  margin-bottom: 10px;
}
</style>
