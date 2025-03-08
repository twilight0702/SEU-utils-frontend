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
    const size = ref(4);
    const tiles = ref(
      Array(16)
        .fill(0)
        .map((_, index) => ({
          value: 0,
          position: index,
        }))
    );
    const score = ref(0);

    const isGameOver = ref(false);
    const tileClass = (tile) => {
      return `tile-${tile.value}`;
    };

    const generateTile = () => {
      let emptyTiles = tiles.value.filter((tile) => tile.value === 0);
      if (emptyTiles.length > 0) {
        let randomTile =
          emptyTiles[Math.floor(Math.random() * emptyTiles.length)];
        randomTile.value = Math.random() > 0.5 ? 2 : 4;
      }
    };

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

    const shiftTiles = (direction) => {
      isGameOver.value = true;
      console.log("before:", tiles.value);
      let moved = false;
      //改成使用纯数字数组进行计算，最后再放回响应数组，避免操作position
      let tempTiles = [];
      for (let i = 0; i < tiles.value.length; i++) {
        tempTiles.push(tiles.value[i].value);
      }
      let start = tempTiles;
      console.log("beforeValue:", tempTiles);

      // Rotate the tiles to simplify the movement logic (only shift left)
      if (direction === "right" || direction === "left") {
        if (direction === "right") {
          tempTiles = tempTiles.reverse();
        }
        for (let row = 0; row < size.value; row++) {
          let start = row * size.value;
          let rowTiles = tempTiles.slice(start, start + size.value);
          if (rowTiles) {
            let newRow = mergeTiles(rowTiles);
            tempTiles.splice(start, size.value, ...newRow);
          }
        }
        if (direction === "right") {
          tempTiles = tempTiles.reverse();
        }
      } else {
        let transposedTiles = transpose(tempTiles);
        if (direction === "down") {
          transposedTiles = transposedTiles.reverse();
        }
        for (let col = 0; col < size.value; col++) {
          let start = col * size.value;
          let colTiles = transposedTiles.slice(start, start + size.value);
          if (colTiles) {
            let newCol = mergeTiles(colTiles);
            transposedTiles.splice(start, size.value, ...newCol);
          }
        }
        if (direction === "down") {
          transposedTiles = transposedTiles.reverse();
        }
        tempTiles = transpose(transposedTiles);
      }

      console.log("处理后的value:", tempTiles);

      for (let i = 0; i < tiles.value.length; i++) {
        tiles.value[i].value = tempTiles[i];
      }

      console.log("处理后的Tiles:", tiles.value);

      checkGameOver();
      generateTile();

      console.log("after:", tiles.value);
    };

    const mergeTiles = (tiles) => {
      console.log("before merge:", tiles);
      let mergedTiles = [];

      // Merge the tiles
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

      // 判断是否发生变化，如果有变化则继续递归合并
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

    const transpose = (matrix) => {
      console.log("before transpose:", matrix);
      if (!Array.isArray(matrix)) {
        console.error("Invalid matrix for transpose:", matrix);
        return [];
      }

      // Step 1: Convert the flat array into a 2D array (matrix)
      let twoDArray = [];
      for (let i = 0; i < matrix.length; i += size.value) {
        twoDArray.push(matrix.slice(i, i + size.value));
      }
      console.log("in transpose:", twoDArray);

      // Step 2: Perform the transpose operation (swap rows and columns)
      let transposedArray = [];
      for (let col = 0; col < size.value; col++) {
        for (let row = 0; row < twoDArray.length; row++) {
          transposedArray.push(twoDArray[row][col]);
        }
      }
      console.log("after transpose:", transposedArray);

      return transposedArray;
    };

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
