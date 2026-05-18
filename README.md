# Digital Card — Rust 电子名片模板

Rust 实现的交互式电子名片，粒子组成的 ASCII 点阵艺术字 + 浮动技能标签。

鼠标/触屏按住将粒子驱散，松手后弹簧物理拉回原位。

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.70%2B-orange">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS%20%7C%20Android-blue">
  <img src="https://img.shields.io/badge/binary-~820KB-green">
  <img src="https://img.shields.io/badge/license-MIT-lightgrey">
</p>

## 效果

- 启动即展示个人字符画，这里用 Misser 艺术字示例
- **按住鼠标 / 触屏**，粒子从光标处散射，靠近光标的粒子产生辉光
- **松开**后弹簧模型将粒子拉回原位
- 技能标签围绕中心做正弦浮动，深色背景 + 网格线

## 快速开始

```bash
git clone https://github.com/yourname/digital-card.git
cd digital-card
cargo run --release
```

## 使用自定义 ASCII 艺术（.txt 文件）

支持通过命令行加载任意 ASCII 艺术文本文件，渲染为点阵粒子：

```bash
# 使用 jp2a 从图片生成 ASCII 文本，或手写 ASCII 艺术文件
./digital-card my_art.txt
```

### ASCII 艺术文件格式

纯文本文件，空格为空白，非空格字符**按视觉密度**映射为不同亮度的 `.` 粒子：

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

**示例 `my_art.txt`：**

```
    @@@@@@
   @@@@@@@@
  @@@@@@@@@@
 @@@@@@@@@@@@
@@@@@@@@@@@@@@
 @@@@    @@@@
 @@@@    @@@@
 @@@@    @@@@
 @@@@    @@@@
 @@@@@@@@@@@@
  @@@@@@@@@@
```

### 用 jp2a 生成 ASCII 文本

```bash
# 安装 jp2a (Ubuntu/Debian)
sudo apt install jp2a

# 将图片转为 ASCII 文本，存入 .txt
jp2a photo.jpg --width=80 > art.txt

# 在电子名片中加载
./digital-card art.txt
```

建议生成宽度 40\~120 字符、高度 15\~80 行的 ASCII 文本以获得最佳点阵效果。

## 自定义

编辑 `src/main.rs`：

| 项目 | 行号区域 | 说明 |
|------|----------|------|
| 默认名字 | `generate_ascii_art_from_letters("MISSER")` | 改为你的名字 |
| 技能标签 | `skill_names` 数组（20 个槽位，留空为 `""`） | 替换技能列表 |
| 字符大小 | `CHAR_SIZE` 常量 | 调整点的大小 |
| 散开半径 | `REPULSION_RADIUS` | 鼠标影响范围 |
| 弹簧力度 | `SPRING_K` | 粒子回弹速度 |
| 颜色主题 | `dot_color` 函数 | 修改 RGB |
| 窗口大小 | `window_width/height` | 默认窗口尺寸 |

如需添加新的字母形状（中文字等），在 `letter_pattern` 函数按 14×18 位图格式（`1` 填充、`.` 留空）添加定义，程序自动做上采样 + 模糊平滑。

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
- 程序化 ASCII 点阵生成 (位图 → 上采样 → Box Blur → 亮度映射)

## License

MIT
