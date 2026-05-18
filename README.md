# Digital Card — Rust 电子名片模板

Rust 实现的交互式电子名片，ASCII 点阵字符画 + 浮动技能标签。

鼠标/触屏按住将粒子驱散，松手后弹簧物理拉回原位。

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.70%2B-orange">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS%20%7C%20Android-blue">
  <img src="https://img.shields.io/badge/binary-~820KB-green">
  <img src="https://img.shields.io/badge/license-MIT-lightgrey">
</p>

## 效果

- 启动加载 `art.txt` 中的 ASCII 字符画，用 `.` 粒子组成点阵展示
- **按住鼠标 / 触屏**，粒子从光标处散射，靠近光标的粒子产生辉光
- **松开**后弹簧模型将粒子拉回原位
- 技能标签围绕中心做正弦浮动，深色背景 + 网格线

## 快速开始

```bash
git clone https://github.com/yourname/digital-card.git
cd digital-card
cargo run --release
```

`art.txt` 已随项目提供。启动即自动加载。

## ASCII 艺术文件规范

### 文件命名

| 加载方式 | 说明 |
|----------|------|
| `./digital-card` | 自动查找当前目录下的 **`art.txt`** |
| `./digital-card my_art.txt` | 加载指定路径文件 |
| `./digital-card /path/to/file.txt` | 加载任意路径文件 |

### 文件格式

纯文本，空格为空白，非空格字符**按视觉密度**映射为不同亮度的 `.` 粒子：

| 字符密度 | 示例字符 | 粒子亮度 |
|----------|----------|----------|
| 最密 | `@#$%&` | 最亮 |
| 密 | `8BMWN0` | 亮 |
| 中密 | `*+=XZ` | 中等 |
| 中 | `oahkb` | 中等偏暗 |
| 偏淡 | `dpqwm` | 暗 |
| 淡 | `-~:\|/` | 很暗 |
| 最淡 | `.,'` `` ` `` | 极暗 |
| 空白 | (空格) | 不生成粒子 |

### 用 jp2a 生成 ASCII 文本

```bash
# 安装 jp2a (Ubuntu/Debian)
sudo apt install jp2a

# 将图片转为 ASCII 文本
jp2a photo.jpg --width=80 > art.txt

# 在电子名片中加载
./digital-card art.txt
```

建议生成宽度 40~120 字符、高度 15~80 行的 ASCII 文本以获得最佳点阵效果。

如果没有 `art.txt` 文件，程序正常启动但不显示字符画，仅展示技能标签和背景网格。

## 自定义修改速查表

编辑 `src/main.rs`，以下为关键修改位置：

| 项目 | 位置 | 说明 |
|------|------|------|
| 粒子点大小 | `src/main.rs:8` `CHAR_SIZE` | 当前 21.0 |
| 弹簧回弹速度 | `src/main.rs:4` `SPRING_K` | 当前 0.07 |
| 鼠标影响半径 | `src/main.rs:6` `REPULSION_RADIUS` | 当前 180.0 |
| 散开力度 | `src/main.rs:7` `REPULSION_FORCE` | 当前 1100.0 |
| 颜色主题 | `src/main.rs:95` `dot_color` 函数 | 修改 RGB 通道 |
| 窗口大小 | `src/main.rs:104-105` `window_width`/`window_height` | 当前 1300×850 |
| 窗口标题 | `src/main.rs:103` `window_title` | 当前 "Misser - Digital Card" |
| 技能标签列表 | `src/main.rs:164` `skill_names` 数组 | 20 个槽位，留空为 `""` |
| 技能字体大小 | `src/main.rs:279` `measure_text(... 21 ...)` 和 `L281` `21.0` | 当前 21 |
| 背景网格间距 | `src/main.rs:234` `grid_spacing` | 当前 45.0 |
| 背景网格颜色 | `src/main.rs:235` `grid_color` | RGB+Alpha |
| 粒子密度阈值 | `src/main.rs:142` `if d > 0.02` | 越小粒子越多 |
| 提示文字 | `src/main.rs:289-291` `let hint = ...` | 交互提示 |
| 粒子水平间距 | `src/main.rs:130` `CHAR_SIZE * 0.33` | 越小越密 |
| 粒子垂直间距 | `src/main.rs:131` `CHAR_SIZE * 0.85` | 越小越密 |
| 技能圆环半径 | `src/main.rs:177` `screen_width()... * 0.38` | 技能距中心距离 |

## 交叉编译

```bash
# Windows .exe
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

# Android APK
cargo install cargo-apk
cargo apk build --release
```

## 技术栈

- [macroquad](https://github.com/not-fl3/macroquad) — 跨平台 2D 渲染
- 粒子弹簧物理 (胡克定律) + 斥力场交互
- ASCII 点阵生成 (字符密度 → 亮度映射 + Box Blur 平滑)

## License

MIT
