# **SEU-utils-frontend**

## **简介**

**SEU-utils-frontend** 是 **SEU-utils** 的桌面应用版本，基于 **Vue3** 和 **Tauri** 开发，提供 **可视化的绩点计算器** 和 **2048 游戏**。

### **主要特点**

 ✅ 现代化 UI，界面美观
 ✅ 绩点计算器：自动获取课程成绩，可筛选课程动态计算绩点
 ✅ 2048 游戏：本地存储最高分记录

------

## **功能介绍**

### 🎓 **绩点计算器**

- **自动获取成绩**：输入 **教务网站 Cookie**，自动获取所有课程成绩（应用内有详细教程，这里不多赘述）。

- **灵活筛选**：

  - **学期筛选**：默认选择所有学期，可自由选择学期。
  - **课程类型筛选**：默认筛选 **必修** 和 **限选**，可自由切换。
  - **单门课程控制**：勾选或取消某门课程，实时计算对绩点的影响。

- **本地缓存**：成绩数据本地存储，会默认优先读取本地缓存数据（后续可重新导入），读取路径：

  ```
  C:\Users\你的用户名\AppData\Roaming\com.seu.utils.frontend\gpa-data\grades.json
  ```

  **示例截图**：

  ![要脸给自己成绩打码了](.\images-for-readme\S1.png)

  ![打码2.0](.\images-for-readme\S2.png)

  

------

### 🎮 **2048 游戏**

- **键盘操作**：支持键盘方向键进行操作。

- **本地最高分记录**：存储在本地，路径：

  ```
  C:\Users\你的用户名\AppData\Roaming\com.seu.utils.frontend\2048\2048-max-score.txt
  ```

  **示例截图**：

  ![游戏截图](.\images-for-readme\S4.png)

  

------

## **安装与运行**

### **📥 下载**

1. **前往 [Releases](https://github.com/twilight0702/SEU-utils-frontend/releases) 页面**，下载最新的 `setup.exe` 安装包。
2. **运行安装包**，按照提示完成安装。

### **🚀 启动**

- 安装完成后，可在 **开始菜单** 或 **桌面快捷方式** 启动 `SEU-utils-frontend`。

------

## **技术栈**

| 组件         | 技术                |
| ------------ | ------------------- |
| **前端框架** | Vue3 + Vite         |
| **桌面应用** | Tauri (Rust)        |
| **存储方式** | JSON 、TXT 本地缓存 |

---

## **本地开发**

如果你希望在本地开发或贡献代码，请按照以下步骤进行：  

### **📌 初始环境**  
在开始之前，请确保你已安装以下依赖：  
- **Node.js**（建议使用最新 LTS 版本）  
- **Rust**（用于 Tauri 框架，安装方式见下方）  
- **Tauri CLI**（用于构建 Tauri 应用）  

#### **安装 Rust**

如果你尚未安装 Rust，可以参考[Rust语言圣经](https://course.rs/first-try/installation.html)进行安装

#### **安装 Tauri CLI**

运行以下命令安装 Tauri CLI：
```sh
npm install -g @tauri-apps/cli
```

---

### **⚙️ 开发环境搭建**

1. **克隆本项目**
   
2. **安装依赖**
   ```sh
   npm install
   ```

3. **运行开发环境**
   
   ```sh
   npm run tauri dev
   ```
   
4. **构建 Windows 版本**
   ```sh
   npm run tauri build  # 生成 Windows 安装包
   ```

---

## **贡献**

欢迎任何贡献！如果你有想法加入其他常用工具或发现 bug，请按照以下方式参与：

1. **提 Issue**：在 [Issues](https://github.com/你的仓库/issues) 页面提交问题或建议。
2. **Fork & 提交 PR**：Fork 本仓库后，提交 PR 进行代码贡献。
3. **讨论**：有任何问题可在 Issue 下交流。

------

## **其他**

- **当前版本仅支持 Windows**
- **如有 bug 或建议，请提交 Issue！** 🎉
