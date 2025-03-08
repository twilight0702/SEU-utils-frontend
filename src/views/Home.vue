<template>
    <div>
        <button @click="goToGPA">绩点计算器</button>
        <button @click="goto2048">2048</button>
        <div>123</div>
    </div>
</template>
<script setup>
import { useRouter } from 'vue-router';
const router = useRouter();
const goToGPA = () => {
  router.push('/gpa'); // 跳转到绩点计算器页面
};

const goto2048 = () => {
  router.push('/game-2048'); // 跳转到2048页面
};
</script>
<style scoped>
</style>