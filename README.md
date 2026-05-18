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

## 中文技能标签

技能标签支持中文，`skill_names` 数组（`src/main.rs:185`）中的内容已改为 `String` 类型，可直接写入中文：

```rust
let skill_names: &[&str] = &[
    "Rust", "Python", "机器学习", "Web 开发",
    "Linux", "Docker", "Git", "SQL",
    "云计算", "开源", "Go", "React",
    "C++", "K8s", "CI/CD", "API 设计",
    "", "", "", "",
];
```

### CJK 字体配置

中文渲染需要系统安装 CJK 字体。程序会自动搜索以下路径（`src/main.rs:112-127`）：

| 系统 | 自动搜索路径 |
|------|-------------|
| Linux | `/usr/share/fonts/noto-cjk/NotoSansCJK-*.ttc` |
| Linux | `/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc` |
| Linux | `/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf` |
| macOS | `/System/Library/Fonts/PingFang.ttc` |
| Windows | `C:\Windows\Fonts\msyh.ttc` / `simsun.ttc` |

**Linux 安装 CJK 字体：**

```bash
# Ubuntu/Debian
sudo apt install fonts-noto-cjk

# Arch
sudo pacman -S noto-fonts-cjk
```

未找到 CJK 字体时，中文技能标签会显示为空白/方块，英文标签正常。

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
| 窗口标题 | `src/main.rs:103` `window_title` | 当前 "Digital Card" |
| 技能标签列表 | `src/main.rs:185` `skill_names` 数组 | 20 个槽位，支持中文 |
| 技能字体大小 | `src/main.rs:308` `21` 和 `L304` `21` | 当前 21 |
| CJK 字体路径 | `src/main.rs:112` `load_cjk_font` 函数 | 搜索路径列表 |
| 背景网格间距 | `src/main.rs:255` `grid_spacing` | 当前 45.0 |
| 背景网格颜色 | `src/main.rs:256` `grid_color` | RGB+Alpha |
| 粒子密度阈值 | `src/main.rs:163` `if d > 0.02` | 越小粒子越多 |
| 粒子水平间距 | `src/main.rs:151` `CHAR_SIZE * 0.33` | 越小越密 |
| 粒子垂直间距 | `src/main.rs:152` `CHAR_SIZE * 0.85` | 越小越密 |
| 技能圆环半径 | `src/main.rs:198` `screen_width()... * 0.38` | 技能距中心距离 |

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
- 系统 CJK 字体自动检测加载

## License

MIT
