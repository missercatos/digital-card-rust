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

- 启动加载 `art.txt` 中的 ASCII 字符画，按原始字符展示（支持中文等 Unicode 字符，需系统安装 CJK 字体）
- **按住鼠标 / 触屏**，粒子从光标处散射，靠近光标的粒子产生辉光
- **松开**后弹簧模型将粒子拉回原位
- 技能标签围绕中心做正弦浮动，深色背景 + 网格线
- 支持自定义字符画尺寸限制、间距、字号、技能标签间隔等参数（详见下方速查表）

## 快速开始

```bash
git clone https://github.com/yourname/digital-card.git
cd digital-card
cargo run --release
```

`art.txt` 已随项目提供。启动即自动加载。

## 导出二进制文件

编译后直接分发二进制 + `art.txt` 即可运行，无需安装 Rust 环境：

```bash
# 编译 release 二进制
cargo build --release

# 二进制位置：target/release/digital-card
# 将 digital-card 和 art.txt 放到同一目录下，直接执行：
./digital-card
```

分发时只需携带两个文件：

```
digital-card/          (或任意目录)
├── digital-card       (可执行文件)
└── art.txt            (ASCII 艺术文件)
```

双击或终端执行 `./digital-card` 即可展示，效果与 `cargo run --release` 完全一致。`art.txt` 也支持替换为自定义文件：

```bash
./digital-card my_card.txt
```

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

纯文本 UTF-8 文件，空格为空白，非空格字符**按视觉密度**映射为不同亮度的粒子。支持中文、日文等 Unicode 字符（需系统 CJK 字体）。

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

字符画宽高超过 `limit_w` / `limit_h` 时自动等比缩小（字号和间距同步压缩，下限 0.25）。

矩形字符画四角自动做圆弧裁剪（`parse_ascii_art` 内，角半径 `min(w,h) * 0.15`）。

## 自定义修改速查表

编辑 `src/main.rs`，以下为关键修改位置：

### 字符画尺寸与间距

| 项目 | 位置 | 说明 |
|------|------|------|
| 字符基础字号 | `L10` `CHAR_SIZE` | 当前 21.0，压缩时自动等比缩小 |
| 水平间距系数 | `L211` `sx_base` | 当前 `CHAR_SIZE * 0.58` |
| 垂直间距系数 | `L212` `sy_base` | 当前 `CHAR_SIZE * 1.05` |
| 最大宽度限制 | `L213` `limit_w` | 当前 1000.0 px，超限触发压缩 |
| 最大高度限制 | `L214` `limit_h` | 当前 750.0 px，超限触发压缩 |
| 角圆弧半径 | `L96` `cr` | 当前 `min(w,h) * 0.15` |
| 密度采样阈值 | `L235` `if d > 0.02` | 越小粒子越多 |

压缩规则：当 `raw_w > limit_w` 或 `raw_h > limit_h` 时，间距和字号按 `min(limit_w/raw_w, limit_h/raw_h)` 等比例缩小（下限 0.25）。

### 技能标签

| 项目 | 位置 | 说明 |
|------|------|------|
| 技能列表 | `L255` `skill_names` | 20 个槽位，支持中文，留空为 `""` |
| 标签字号 | `L398` `measure_text(... 21 ...)` | 当前 21 |
| 标签圆环半径 | `L268` `screen_width()... * 0.56` | 越大标签距中心越远 |

### 物理与交互

| 项目 | 位置 | 说明 |
|------|------|------|
| 聚合速度 | `L6` `SPRING_K` | 弹簧拉力，值越大粒子回弹越快，当前 0.15 |
| 逸散速度 | `L9` `REPULSION_FORCE` | 斥力强度，值越大粒子散开越快，当前 1800.0 |
| 振荡收束 | `L7` `DAMPING` | 速度衰减系数，越小振荡越快停止，当前 0.82 |
| 鼠标影响半径 | `L8` `REPULSION_RADIUS` | 光标影响范围，当前 180.0 |

### 视觉与窗口

| 项目 | 位置 | 说明 |
|------|------|------|
| 颜色主题 | `L140` `dot_color` 函数 | 修改 RGB 通道 |
| 窗口尺寸 | `L149-150` `window_width`/`window_height` | 当前 1500×900 |
| 窗口标题 | `L148` `window_title` | 当前 "Digital Card" |
| 背景网格间距 | `L325` `grid_spacing` | 当前 45.0 |
| 背景网格颜色 | `L326` `grid_color` | RGB+Alpha |

### 其他

| 项目 | 位置 | 说明 |
|------|------|------|
| 内置艺术字 | `L4` `EMBEDDED_ART` | 编译期嵌入的默认文本 |
| CJK 字体路径 | `L157` `load_cjk_font` 函数 | 自动搜索的字体路径列表 |
| ASCII 字符密度表 | `L41` `parse_ascii_art` | 字符→亮度映射 |

## 交叉编译 & 移动端部署

### 桌面端

```bash
# Windows .exe
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

# macOS (在 macOS 上编译)
cargo build --release
```

### Android（APK）

编译为 APK 安装包，可直接安装运行。`art.txt` 已编译进 APK 内置。

```bash
# 1. 安装 Android 工具链
rustup target add aarch64-linux-android
cargo install cargo-apk

# 2. 安装 Android SDK + NDK
# 推荐安装 Android Studio，或单独安装 sdkmanager / ndk
# 设置环境变量：
#   export ANDROID_SDK_ROOT=~/Android/Sdk
#   export ANDROID_NDK_ROOT=~/Android/Sdk/ndk/27.0.12077973

# 3. 编译 APK
cargo apk build --release

# 4. APK 位于 target/release/apk/DigitalCard.apk
# 传到手机安装即可
adb install target/release/apk/DigitalCard.apk
```

### Android（Termux，免 APK）

在手机上安装 [Termux](https://termux.com) 后，通过它运行 Linux ARM 二进制：

```bash
# 交叉编译 ARM64 静态二进制
rustup target add aarch64-unknown-linux-musl
cargo build --release --target aarch64-unknown-linux-musl

# 将 target/aarch64-unknown-linux-musl/release/digital-card 和 art.txt
# 传到手机，在 Termux 中执行：
#   chmod +x digital-card
#   ./digital-card
```

> Termux 需要安装 `x11-repo` 和 `termux-x11` 才能显示图形窗口。
> 更简单的方式：直接使用上面的 APK 方式。

### iOS

iOS 不支持侧载原生二进制。如需要，可通过 Xcode 将 Rust 编译为静态库嵌入 Swift/ObjC 壳工程，需 Mac + Apple Developer 账号（$99/年）。

## 技术栈

- [macroquad](https://github.com/not-fl3/macroquad) — 跨平台 2D 渲染
- 粒子弹簧物理 (胡克定律) + 斥力场交互
- ASCII 点阵生成 (字符密度 → 亮度映射 + Box Blur 平滑)
- 系统 CJK 字体自动检测加载

## License

MIT
