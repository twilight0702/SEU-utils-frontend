<template>
  <div class="wrapper">
    <h1 :class="titleClass" @click="playEffect">东南大学常用工具箱🎉</h1>
    <p style="color: gray">点一下标题试试呢 :D</p>

    <div class="container">
      <Button @click="goToGPA">绩点计算器</Button>
      <br />
      <Button @click="goto2048">2048</Button>
    </div>
  </div>
</template>

<script setup>
import { useRouter } from "vue-router";
import "animate.css";
import { ref } from "vue";

//标题动画效果
const titleClass = ref(""); // 绑定 h1 的动态类

const playEffect = () => {
  titleClass.value = "animate__animated animate__tada"; // 触发 animate.css 的 "tada" 动画
  setTimeout(() => {
    titleClass.value = ""; // 1s 后移除动画，方便下次点击生效
  }, 1000);
};

//路由跳转函数
const router = useRouter();
const goToGPA = () => {
  router.push("/gpa"); // 跳转到绩点计算器页面
};

const goto2048 = () => {
  router.push("/game-2048"); // 跳转到2048页面
};
</script>

<style scoped>
.container {
  width: 50vw;
  height: auto; /* 让内容决定高度 */
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center; /* **垂直居中** */

  margin-left: auto;
  margin-right: auto;
  margin-top: 40px;

  border: #69a197 2px solid;
  padding: 50px;
  border-radius: 10px;
}

/* 按钮样式 */
button {
  width: 250px;
  padding: 12px 24px;
  font-size: 30px;
  font-weight: bold;
  color: white;
  background-color: #69a197; /* 蓝色背景 */
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: transform 0.2s ease-in-out, background-color 0.2s;
}

/* 鼠标悬停时放大 */
button:hover {
  transform: scale(1.1); /* 原地放大，不影响布局 */
  background-color: #e5c94c; /* 悬停时颜色变深 */
}

.wrapper {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
}

h1 {
  font-size: 50px;
  color: #69a197;
  cursor: pointer;
  transition: color 0.5s ease-in-out;
  margin-top: -30px;
}

h1:hover {
  color: #e181a9; /* 鼠标悬停时，h1 颜色变为粉色 */
}
</style>
