# GXDE 文件管理器外观对照

Dlea 的界面目标是 [gxde-file-manager](https://github.com/GXDE-OS/gxde-file-manager)（`char/qt6_migration`，Deepin 文件管理器 V4 的 GXDE 分支）的日常窗口。实现留在现有 GPUI 结构里，不改列表模型、视口统计和后端协议。

颜色和尺寸来自该仓库的：

- `gxde-file-manager-lib/themes/dark/DFileManagerWindow.theme`
- `gxde-file-manager-lib/themes/light/DFileManagerWindow.theme`
- `gxde-file-manager-lib/themes/dark/config.ini`（侧栏 `BookmarkItem`）
- `gxde-file-manager-lib/themes/*/DFileView.theme`、`DStatusBar.theme`
- `gxde-file-manager-lib/views/dtoolbar.cpp`、`dstatusbar.cpp`

没有把 GPL 的 SVG、PNG 或 QSS 文件拷进本仓库。按钮字形仍是 Dlea 自己的描边图标。主题色走现有 `colors.toml` / `shell.toml` 合同；两边都没有时，窗口使用 GXDE 深色底 `#202020` 和强调色 `#2ca7f8`。

## 已对齐（相对源码，不是并排像素）

| 区域 | GXDE | Dlea |
|---|---|---|
| 工具栏顺序 | 侧栏、6 px、后退\|前进\|向上贴在一起、10 px、面包屑、10 px、搜索、3 px、视图托盘、3 px、设置 | 同序。网格视图仍把搜索换成筛选和排序，这是原有行为 |
| 按钮尺寸 | 24×24，圆角 4 | 默认字号下命中盒宽 24。顶栏高度仍是现有 27 px（GXDE 地址条高 40），圆角 4 |
| 深色按钮 | 静止白 15%、悬停 30%、按下 10%、禁用 6%；选中为实色强调色 | 同 alpha。选中图标仍用主题前景色，以保持原有标记对比度测试 |
| 浅色按钮 | 白底、10% 描边，悬停为蓝色渐变 | 白底和描边。悬停是强调色 16% 淡洗，而不是 `#8CCFFF→#4BB8FF` 渐变，这样深色图标仍可读 |
| 面包屑 | 深色井 `rgba(255,255,255,0.15)`，浅色白底描边，当前段实色强调色、白字、圆角 4 | 同井和当前段。分隔符仍是 `/`，不是 GXDE 的箭头 |
| 视图托盘 | `rgba(0,0,0,0.1)`，内边距 1，按钮贴齐 | 同。托盘里保留列表、分栏、图标、双栏四个模式 |
| 侧栏行 | 高 30、图标 16、左边距 13；选中 `#332ca7f8`，悬停 `#1Affffff` | 已按这组数字和 alpha |
| 列表选中 | 强调色 76/255 填充、150/255 描边 | 列表圆角 2，图标视图圆角 3 |
| 列头 | `QHeaderView` 黑 10% | 同 |
| 状态栏 | 项数居中，右侧 120 px 缩放滑块、右边距 20 | 项数居中，滑块保留。磁盘余量仍在滑块左侧，GXDE 默认状态栏没有这一项 |
| 标签页 | 方角、约 8% 底 | 沿用已落地的 GXDE 标签底色 |
| 菜单 | 强调色 20% 高亮，圆角约 6 | 沿用已落地的菜单高亮 |

## 未对齐

- 窗口装饰不是 DTK `DTitlebar`。最小化、最大化和关闭仍由 GPUI / 合成器绘制，没有 GXDE 那种标题栏右侧的红悬停关闭按钮。
- 顶栏高度是 27 px（正文字号 14），不是地址条的 40 px。把顶栏加高会改动整窗几何和大量既有测试。
- 图标不是 `themes/dark/icons` 里的那套彩色侧栏和工具栏 SVG。那些资源是 GPL-3.0，没有并入 MIT 的 Dlea。
- 浅色悬停不是 QSS 里的蓝渐变，选中工具栏图标也不是强制白色。
- 面包屑分隔是 `/`，当前段圆角是四角而不是仅右侧。
- 视图托盘比 GXDE 多出分栏和双栏。网格模式的筛选、排序仍占搜索按钮的位置。
- 状态栏仍显示磁盘余量和操作消息；项数字体颜色改为次级色。
- 对话框、滚动条和拖放跟手动画没有逐控件对照到 DTK。对话框圆角继续跟合成器的 `decoration.rounding`。
## 真实安装（2026-11-15）

镜像：`https://mirrors.hit.edu.cn/GXDE/gxde-os/`。主机是 Ubuntu 24.04 amd64，不能直接装这些包：它们要求 `libqt6core6t64 >= 6.8.2`，以及 `qt6-base-private-abi (= 6.8.2)`。Noble 的 Qt 更旧，没有替换主机的 Qt。

用的是一次性 Debian 13（trixie）chroot：`/var/tmp/gxde-chroot`，`mmdebstrap --variant=minbase`，源是 `http://deb.debian.org/debian`。chroot 里的 `libqt6core6t64` 是 `6.8.2+dfsg-9+deb13u2`，Provides `qt6-base-private-abi (= 6.8.2)`。

签名：

- `zhuangzhuang/InRelease` 和 `hetao/InRelease` 都由 `BCC6 D7F0 7130 C0FA 3266 9066 FD6E EA1F 20CD 4B27`（`gfdgd_xi <3025613752@qq.com>`，从 Ubuntu keyserver 导入）签过，`gpg --verify` 为 Good signature。这把钥匙没有 web-of-trust 认证。
- 已下载的 `zhuangzhuang/Packages.xz` SHA256 等于签名的 Release：`1918d98e94b25388adfd50a206a0c7247778b18f4cdaf1e73e8eed38027417e4`。
- `hetao/Packages.xz` SHA256 等于签名的 Release：`f98efc795e86eeaec3ac23f602b5877a75fdf30f69c32cb82f3c90f9c28cc00b`。

chroot 的 apt 源（signed-by，没有 `trusted=yes`）：

```
deb [signed-by=/etc/apt/keyrings/gxde-archive.gpg] https://mirrors.hit.edu.cn/GXDE/gxde-os/zhuangzhuang/ ./
```

实际装上的版本（`apt-get install --no-install-recommends`）：

| 包 | 版本 | 来源 |
|---|---|---|
| gxde-file-manager | 6.4.21 amd64 | zhuangzhuang |
| libgxde-file-manager | 6.4.21 amd64 | zhuangzhuang |
| libdtk2widget6 | 6:6.3.9 amd64 | zhuangzhuang |
| libdtk6core | 6.0.48 amd64 | zhuangzhuang |
| libgxde-bernese-qt6-0 | 1.0.3-gxde5 amd64 | zhuangzhuang |
| gxde-compressor | 1.6.0 all | zhuangzhuang |
| gxde-icon-theme | 2026.03.20 all | zhuangzhuang |
| libqt6core6t64 | 6.8.2+dfsg-9+deb13u2 | Debian 13 |

另外装了 `fonts-noto-core`、`fonts-noto-cjk`、`xvfb`、`openbox`、`xdotool`、`imagemagick`、`libxcb-cursor0`。没有装完整桌面。

没装上的关键包：

- `dde-qt6integration`（zhuangzhuang `6.0.35-gxde2`）依赖 `dde-qt6xcb-plugin`，而 zhuangzhuang 的索引里没有这个包。
- hetao 有 `dde-qt6xcb-plugin_6.0.48-2_amd64.deb`，SHA256 `70d2175e7c1d603748a8e6d4f0a34168a1bab94304d80965abca0896313d7647` 与签名索引一致，但依赖 `qt6-base-private-abi (= 6.8.0)`。trixie 只提供 `6.8.2`。没有用 `--force-depends`。

源码 `dfilemanagerwindow.cpp` 里，地址栏只在 `isDXcbPlatform()` 为真时放进 `DTitlebar`。没有这个插件时，真实进程画出的窗口就没有后退/前进/面包屑。这是这次运行的真实结果，不是 Dlea 该去模仿的残缺布局。

运行：chroot 内 `Xvfb :99 -screen 0 1280x800x24`，`openbox`，`LANG=en_US.UTF-8`，`QT_FONT_DPI=96`，`gxde-file-manager /opt/fixture`，窗口调到 1100×700。Dlea 用同一块 X11 socket（bind `/tmp/.X11-unix`）、同一夹具目录的宿主副本 `/tmp/gxde-fixture`、`target/debug/flea --gui`，窗口同样 1100×700。夹具只有临时文件：Desktop、Documents、Downloads、Music、Pictures、Videos、notes.txt、readme.md。

真实 GUI 里做成的事：

- 点侧栏 Documents：行变成浅蓝、文字变蓝，窗口标题变成 `Documents — Deepin File Manager`，空目录显示 Folder is empty。
- 空白处右键：New folder、New document、Display as、Sort by、Refresh、Open in new window as admin、Open in terminal、Copy Path、Paste（灰）、Select all、Properties。
- Ctrl+2：详情列 Name / Time changed / Size / Type；选中行是整行浅蓝。目录的 Size 是 `-`。
- 右键 readme.md：名字后面是圆角浅蓝选中块。菜单是 Open、Open with、Cut、Copy、Rename、Delete、Compress、Create link、Send to desktop、Properties。
- 点 Properties：弹出一个灰头白身的对话框（Basic info）。路径文字挤成了 `optfixturereadme.md`，这是该环境下对话框自己的显示问题。
- Alt+Left 没有回到上一级。没有 dxcb 时工具栏不在树上，历史按钮也不在。

同一显示器上的 Dlea：

- 列表画出这 8 项。状态栏在列出之后是 `8 items`，右侧有磁盘余量和缩放滑块。
- 顶栏有侧栏按钮、后退/前进/向上、蓝色当前路径 `/tmp/gxde-fixture`、搜索、四个视图按钮、设置。
- 右键行菜单：Open、Open With、Cut、Copy、Paste、Duplicate、Rename、Properties、Permissions、Add to Favorites、Tags、Move to Trash。
- 点视图按钮没有切到图标视图（点击落在列表上，选中了 Desktop）。图标视图的对照仍是 GXDE 的真实图标视图对 Dlea 的列表。

无头 GPUI 单测截图和这次 X11 真窗口是两回事。无头图只证明 Dlea 自己的绘制；上面这些才是两个进程在同一 Xvfb 上的界面。

## 和真实窗口比，仍然差的

- 真实 6.4.21 在没有 dxcb 时没有地址栏。源码要求地址栏嵌在标题栏里，高 40。Dlea 把同一组控件画在客户区，高 27。不是把工具栏删掉去迁就这次缺插件的窗口。
- 真实默认是浅色。Dlea 在没有主题文件时用深色 `#202020`，这是 nosDshell 的深色配方，不是这次二进制在无主题服务时的浅色。
- 侧栏：真实窗口是 Recent、Home、Desktop、Videos、Music、Pictures、Documents、Downloads、Trash，然后 Computer、System Disk、Data Disk、Computers in LAN。选中是整行浅蓝、蓝字。Dlea 这次只画出 Home、Trash 和一个名为 cursor 的地点。
- 详情列：真实是 Name、Time changed、Size、Type。Dlea 是 Name、Size、Modified。
- 右键菜单项不同，见上。
- 真实图标选中是名字上的圆角浅蓝块。Dlea 的图标视图这次没有在真窗口里切出来。
- 窗口按钮是 Openbox 画的，不是 DTitlebar。Dlea 的标题栏也是窗口管理器的框，里面没有最小/最大/关闭。
