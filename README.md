# Digital Card — Rust 电子名片

一个用 Rust 编写的交互式电子名片程序。ASCII 字符画由粒子组成，鼠标/触屏悬停时粒子散开，移开后自动复原。技能标签以浮动动画环绕。

<br>
<p align="center">
  <img src="https://img.shields.io/badge/rust-1.70%2B-orange" alt="rust version">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS%20%7C%20Android-blue" alt="platform">
  <img src="https://img.shields.io/badge/binary-~800KB-green" alt="binary size">
  <img src="https://img.shields.io/badge/license-MIT-lightgrey" alt="license">
</p>

## 效果演示

- 打开即显示由 ASCII 字符粒子组成的 "MISSER" 艺术字（jp2a 风格）
- **点击并按住鼠标**（或触屏），粒子会从光标位置散开，靠近光标的粒子产生辉光效果
- **松开鼠标**，弹簧物理系统将粒子拉回原位
- 用户技能标签围绕中心艺术字做正弦浮动动画
- 深色背景 + 网格线，类似终端/黑客风格

## 快速开始

### 编译运行

```bash
git clone https://github.com/yourname/digital-card.git
cd digital-card
cargo run --release
```

### 交叉编译

```bash
# Windows .exe
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

# Android APK
cargo install cargo-apk
cargo apk build --release
```

编译后二进制仅 ~800KB，可直接分发执行。

## 自定义

编辑 `src/main.rs`：

| 自定义项 | 位置 | 说明 |
|---------|------|------|
| 名字文本 | 第 192 行 `let art_text = "MISSER"` | 修改为你的名字 |
| 技能标签 | 第 228 行 `skill_names` 数组 | 替换为你的技能栈 |
| 粒子间距 | 第 21 行 `CHAR_SIZE` | 调整字符大小 |
| 散开半径 | 第 12 行 `REPULSION_RADIUS` | 鼠标影响范围 |
| 弹簧力度 | 第 9 行 `SPRING_K` | 粒子回弹速度 |
| 颜色主题 | 第 155 行 `brightness_to_color` | 修改 RGB 通道值 |
| 窗口尺寸 | 第 182 行 `window_width/window_height` | 调整默认窗口大小 |

### 自定义字母形态

如需支持更多字符（中文等），在 `letter_pattern` 函数（第 33 行）中添加对应字母的位图定义即可。位图格式为 10 行字符串，`1` 表示填充、`.` 表示留空，程序会自动上采样+模糊生成平滑的 ASCII 艺术。

## 技术架构

```
macroquad (跨平台渲染引擎)
├── 程序化 ASCII 生成
│   ├── 字母位图定义 (10×n 网格)
│   ├── 3x 上采样 + Box Blur 平滑
│   └── 亮度 → 字符密度映射 (@%#*+=-:.)
├── 粒子物理系统
│   ├── 弹簧模型 (胡克定律)
│   ├── 斥力场 (鼠标交互)
│   └── 速度阻尼
└── UI 层
    ├── 浮动技能标签 (正弦轨迹)
    └── 背景网格 + 辉光反馈
```

## 依赖

- [macroquad](https://github.com/not-fl3/macroquad) — 跨平台 2D 渲染框架
- 零额外运行时依赖

## License

MIT
