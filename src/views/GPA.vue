<template>
    <h1>绩点计算器</h1>
    <div>
        <textarea placeholder="请在这里输入你的cookie" class="InputCookie" v-model="cookie"></textarea>
        <button @click="fetchGrades2">确认</button>
    </div>
</template>
<script setup>
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';
const cookie=ref('');
async function fetchGrades() {
  try {
    const response = await invoke('fetch_grades', { 
      cookie: cookie.value
    })
    console.log('成绩数据:', response)
  } catch (error) {
    console.error('请求失败:', error)
  }
}

async function fetchGrades2() {
  try {
    const response = await invoke('fetch_grades2', { 
      cookie: cookie.value,
      url: "https://ehall.seu.edu.cn/jwapp/sys/cjcx/modules/cjcx/xscjcx.do"
    })
    console.log('响应数据:', response)
  } catch (error) {
    console.error('请求失败:', error)
  }
}
</script>
<style>
.InputCookie {
    height: 100px;
    width: 500px;
    margin-right: 50px;
    white-space: normal;       /* 允许自动换行 */
    overflow-wrap: break-word; /* 长单词强制换行 */
    overflow-y: auto;          /* 内容超出高度时显示滚动条 */
    resize: none;              /* 禁止用户拖拽调整尺寸 */
}
</style>