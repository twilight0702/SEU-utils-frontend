<template>
  <div class="game-board">
    <router-link to="/" class="back-link">返回主页</router-link>
    <div class="tip">Tip:使用键盘的上下左右进行游戏~</div>
    <div class="grid">
      <div
        v-for="tile in tiles"
        :key="tile.position"
        class="tile"
        :class="tileClass(tile)"
      >
        <span>{{ tile.value === 0 ? "" : tile.value }}</span>
      </div>
    </div>
    <div class="info">
      <p style="font-weight: bold; font-size: 30px">当前得分: {{ score }}</p>
      <p style="font-weight: bold; font-size: 20px">
        历史最高分: {{ maxScore }}
      </p>
    </div>
    <button @click="resetGame" class="restart-button">重新开始</button>
  </div>
</template>

<script>
import { ref, onMounted, onUnmounted } from "vue";
import {
  readTextFile,
  BaseDirectory,
  exists,
  writeTextFile,
  mkdir,
} from "@tauri-apps/plugin-fs";
import * as path from "@tauri-apps/api/path";

export default {
  setup() {
    const size = ref(4); //矩阵大小
    const tiles = ref(
      Array(16)
        .fill(0)
        .map((_, index) => ({
          value: 0,
          position: index,
        })) //数组存储表格中的所有方块，具有值和位置（唯一标识用于表单渲染）
    );
    const score = ref(0); //总得分

    const isGameOver = ref(false); //游戏是否结束

    //配置每一个数字方块的class
    const tileClass = (tile) => {
      return `tile-${tile.value}`;
    };

    //在空白区域随机生成一个数字方块（2或4）
    const generateTile = () => {
      let emptyTiles = tiles.value.filter((tile) => tile.value === 0);
      if (emptyTiles.length > 0) {
        let randomTile =
          emptyTiles[Math.floor(Math.random() * emptyTiles.length)];
        randomTile.value = Math.random() > 0.5 ? 2 : 4;
      }
    };

    //重新开始游戏（重置棋盘并随机生成两个数字方块）
    const resetGame = () => {
      tiles.value = Array(16)
        .fill(0)
        .map((_, index) => ({
          value: 0,
          position: index,
        }));
      score.value = 0;
      isGameOver.value = false;
      generateTile();
      generateTile();
    };

    const moveLeft = () => {
      shiftTiles("left");
    };

    const moveRight = () => {
      shiftTiles("right");
    };

    const moveUp = () => {
      shiftTiles("up");
    };

    const moveDown = () => {
      shiftTiles("down");
    };

    //核心移动逻辑
    const shiftTiles = (direction) => {
      isGameOver.value = true; // 假设游戏结束，若有移动则继续游戏

      console.log("before:", tiles.value);

      // 取出纯值数组，避免操作 position
      let tempTiles = tiles.value.map((tile) => tile.value);
      console.log("beforeValue:", tempTiles);

      // 判断是否需要转置，左右直接处理，上下先转置
      let needTranspose = direction === "up" || direction === "down";
      if (needTranspose) {
        tempTiles = transpose(tempTiles);
      }

      // 处理方向：左/上无需反转，右/下需要先反转
      let reverseNeeded = direction === "right" || direction === "down";
      if (reverseNeeded) {
        tempTiles.reverse();
      }

      // 逐行（或逐列）处理合并逻辑
      tempTiles = processTiles(tempTiles);

      // 处理后如果反转过，需要再反转回来
      if (reverseNeeded) {
        tempTiles.reverse();
      }

      // 处理后如果转置过，需要再转置回来
      if (needTranspose) {
        tempTiles = transpose(tempTiles);
      }

      console.log("处理后的value:", tempTiles);

      // 赋值回 tiles 数组
      tiles.value.forEach((tile, index) => {
        tile.value = tempTiles[index];
      });

      console.log("处理后的Tiles:", tiles.value);

      // 检查游戏是否结束
      checkGameOver();
      generateTile();
      console.log("after:", tiles.value);
    };

    // 通用处理合并逻辑，每一行或每一列传入
    const processTiles = (tiles) => {
      let newTiles = [];
      for (let i = 0; i < size.value; i++) {
        let start = i * size.value;
        let line = tiles.slice(start, start + size.value);
        newTiles.push(...mergeTiles(line));
      }
      return newTiles;
    };

    //左移核心行合并操作（递归进行直到操作前后数组完全相同）
    const mergeTiles = (tiles) => {
      console.log("before merge:", tiles);
      let mergedTiles = [];

      for (let i = 0; i < tiles.length; i++) {
        if (tiles[i] === 0) {
          console.log("空位0跳过");
          continue;
        }
        if (i + 1 < tiles.length && tiles[i] === tiles[i + 1]) {
          console.log("相同合并", tiles[i], tiles[i + 1]);
          mergedTiles.push(tiles[i] * 2);
          score.value += tiles[i] * 2;
          i++; //跳过下一个元素
        } else {
          console.log("不同正常放入", tiles[i]);
          mergedTiles.push(tiles[i]);
        }
      }

      //如果不足，在末尾填充0
      while (mergedTiles.length < size.value) {
        mergedTiles.push(0);
        console.log("填充空位0");
      }

      // 判断是否发生变化
      let tilesChanged = false;
      for (let i = 0; i < tiles.length; i++) {
        if (mergedTiles[i] !== tiles[i]) {
          tilesChanged = true;
          break;
        }
      }

      // 如果有变化，则继续合并
      if (tilesChanged) {
        isGameOver.value = false;
        return mergeTiles(mergedTiles);
      } else {
        return mergedTiles;
      }
    };

    //转置（输入输出均为一维数组）
    const transpose = (matrix) => {
      console.log("before transpose:", matrix);
      if (!Array.isArray(matrix)) {
        console.error("Invalid matrix for transpose:", matrix);
        return [];
      }

      //转成二维数组
      let twoDArray = [];
      for (let i = 0; i < matrix.length; i += size.value) {
        twoDArray.push(matrix.slice(i, i + size.value));
      }
      console.log("in transpose:", twoDArray);

      //得到转置后的一维数组
      let transposedArray = [];
      for (let col = 0; col < size.value; col++) {
        for (let row = 0; row < twoDArray.length; row++) {
          transposedArray.push(twoDArray[row][col]);
        }
      }
      console.log("after transpose:", transposedArray);

      return transposedArray;
    };

    //检查游戏是否结束
    const checkGameOver = () => {
      // 1. 检查是否有空格
      let emptyTiles = tiles.value.some((tile) => tile.value === 0);
      if (emptyTiles) return false; // 还有空格，游戏未结束

      // 2. 检查是否有可合并的相邻格子
      for (let i = 0; i < size.value; i++) {
        for (let j = 0; j < size.value; j++) {
          let currentTile = tiles.value[i * size.value + j];
          // 检查右边
          if (
            j < size.value - 1 &&
            currentTile.value === tiles.value[i * size.value + j + 1].value
          ) {
            return false; // 有可合并的格子，游戏未结束
          }
          // 检查下边
          if (
            i < size.value - 1 &&
            currentTile.value === tiles.value[(i + 1) * size.value + j].value
          ) {
            return false; // 有可合并的格子，游戏未结束
          }
        }
      }

      // 如果没有空格，且没有可以合并的格子，则游戏结束
      alert("Game Over!");
      //把成绩记入本地文件
      (async () => {
        const basePath = await path.appDataDir();
        const dirPath = await path.join(basePath, "2048");
        if (!(await exists(dirPath))) {
          await mkdir(dirPath);
        }
        const filePath = await path.join(dirPath, "2048-max-score.txt");

        if (score.value > maxScore.value) {
          maxScore.value = score.value;
          await writeTextFile(filePath, maxScore.value.toString());
        }
      })();
    };

    // 添加键盘事件监听器
    const handleKeydown = (event) => {
      switch (event.key) {
        case "ArrowLeft":
        case "a":
          moveLeft();
          break;
        case "ArrowRight":
        case "d":
          moveRight();
          break;
        case "ArrowUp":
        case "w":
          moveUp();
          break;
        case "ArrowDown":
        case "s":
          moveDown();
          break;
      }
    };

    const maxScore = ref(0);
    onMounted(() => {
      resetGame();
      window.addEventListener("keydown", handleKeydown); // 添加事件监听器
      //从文件读取最大分数
      (async () => {
        try {
          const basePath = await path.appDataDir();
          const dirPath = await path.join(basePath, "2048");

          // 确保目录存在
          if (!(await exists(dirPath))) {
            await mkdir(dirPath, { recursive: true });
          }

          const filePath = await path.join(dirPath, "2048-max-score.txt");

          // 检查文件存在性
          if (!(await exists(filePath))) {
            await writeTextFile(filePath, "0");
          }

          // 读取文件内容
          const data = await readTextFile(filePath);
          maxScore.value = parseInt(data) || 0;
        } catch (error) {
          console.error("文件操作失败:", error);
        }
      })();
    });

    onUnmounted(() => {
      window.removeEventListener("keydown", handleKeydown); // 移除事件监听器
    });

    return {
      size,
      tiles,
      score,
      tileClass,
      generateTile,
      resetGame,
      moveLeft,
      moveRight,
      moveUp,
      moveDown,
      shiftTiles,
      mergeTiles,
      transpose,
      checkGameOver,
      maxScore,
    };
  },
};
</script>

<style scoped>
.back-link {
  padding: 8px 16px;
  background: #69a197;
  color: white;
  border-radius: 4px;
  font-weight: bold;
  text-decoration: none;
  transition: background 0.3s;
}

.back-link:hover {
  background: #e5c94c;
  color: #ffffff;
}

.tip {
  margin-top: 20px;
  color: gray;
}
.restart-button {
  height: 50px;
  padding: 8px 16px;
  background: #69a197;
  color: white;
  border: none;
  border-radius: 4px;
  text-decoration: none;
  font-weight: bold;
  transition: background 0.3s;
}

.restart-button:hover {
  background: #e5c94c;
  color: #ffffff;
}

.game-board {
  text-align: center;
}
.grid {
  display: grid;
  grid-template-columns: repeat(4, 100px);
  grid-gap: 10px;
  margin: 20px auto;
  justify-content: center;
}
.tile {
  width: 100px;
  height: 100px;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #ccc;
  font-size: 40px;
  font-weight: bold;
  border-radius: 5px;
  transition: all 0.2s ease;
}
.tile-2 {
  background-color: #eee4da;
}
.tile-4 {
  background-color: #ede0c8;
}
.tile-8 {
  background-color: #f2b179;
}
.tile-16 {
  background-color: #f59563;
}
.tile-32 {
  background-color: #f67c5f;
}
.tile-64 {
  background-color: #f65e3b;
}
.tile-128 {
  background-color: #edcf72;
}
.tile-256 {
  background-color: #edcc61;
}
.tile-512 {
  background-color: #edc850;
}
.tile-1024 {
  background-color: #edc53f;
}
.tile-2048 {
  background-color: #edc22e;
}
.controls {
  margin-top: 20px;
}
button {
  padding: 10px;
  margin: 5px;
  font-size: 16px;
}
.info {
  margin-top: 20px;
  margin-bottom: 10px;
}
</style>
