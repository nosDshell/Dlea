# GXDE 文件管理器外观对照

Dlea 的界面目标是 [gxde-file-manager](https://github.com/GXDE-OS/gxde-file-manager)（`char/qt6_migration`，Deepin 文件管理器 V4 的 GXDE 分支）的日常窗口。实现留在现有 GPUI 结构里，不改列表模型、视口统计和后端协议。

颜色和尺寸来自该仓库的：

- `gxde-file-manager-lib/themes/dark/DFileManagerWindow.theme`
- `gxde-file-manager-lib/themes/light/DFileManagerWindow.theme`
- `gxde-file-manager-lib/themes/dark/config.ini`（侧栏 `BookmarkItem`）
- `gxde-file-manager-lib/themes/*/DFileView.theme`、`DStatusBar.theme`
- `gxde-file-manager-lib/views/dtoolbar.cpp`、`dstatusbar.cpp`

没有把 GPL 的 SVG、PNG 或 QSS 文件拷进本仓库。按钮字形仍是 Dlea 自己的描边图标。主题色走现有 `colors.toml` / `shell.toml` 合同；两边都没有时，窗口使用 Deepin 15 的浅色底 `#ffffff`、前景 `#000000` 和强调色 `#2ca7f8`。已有的 Omarchy 或 `~/.config/dlea/colors.toml` 仍然覆盖它。

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
- 视图托盘比 GXDE 多出分栏和双栏。列表和分栏里搜索在地址槽右端；网格模式的筛选、排序仍在托盘左边。
- 状态栏仍显示磁盘余量和操作消息；项数字体颜色改为次级色。
- 对话框、滚动条和拖放跟手动画没有逐控件对照到 DTK。对话框圆角继续跟合成器的 `decoration.rounding`。

## 真实安装（2026-11-15）——不能当视觉对照

这次 chroot 里的 `gxde-file-manager` 6.4.21 没有画出 DTK 标题栏。地址栏只在 `isDXcbPlatform()` 为真时放进 `DTitlebar`，而 `dde-qt6xcb-plugin` 没有装上（见下）。窗口只有侧栏和内容区，外加 Openbox 的框。**那些截图和“按这个残缺窗口改 Dlea”的结论作废**，不作为布局、工具栏或图标的依据。下面留下安装记录，是为了说明阻塞点，不是为了对照像素。

行为记录仍然只描述那个缺插件的进程实际做了什么，不能外推成完整文件管理器的外观：

- 点侧栏 Documents：行变成浅蓝、文字变蓝，标题变成 `Documents — Deepin File Manager`，空目录显示 Folder is empty。
- 空白处右键：New folder、New document、Display as、Sort by、Refresh、Open in new window as admin、Open in terminal、Copy Path、Paste（灰）、Select all、Properties。
- Ctrl+2：详情列 Name / Time changed / Size / Type；选中行是整行浅蓝。目录的 Size 是 `-`。
- 右键 readme.md：Open、Open with、Cut、Copy、Rename、Delete、Compress、Create link、Send to desktop、Properties。
- Properties 对话框能打开，路径文字挤成了 `optfixturereadme.md`，这是该环境自己的显示问题。
- Alt+Left 没有回到上一级，因为工具栏不在树上。

同一时刻的 Dlea 点击落在列表上，选中了 Desktop，当时没有切到图标视图。后来在客户区里对准图标按钮的中心再点一次，视图切过去了。那一次失败是点到了工具栏下面，不是按钮把事件吞掉。

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

## DDE 15 公开截图（视觉依据）

这些图是下载后看过的，不是凭记忆画的。截图只说明当时屏幕上有什么，不证明点击、菜单或快捷键。

| 来源 | 版本上下文 | 图 | 画面上能确认的 |
|---|---|---|---|
| [linuxdeepin/developer-center#416](https://github.com/linuxdeepin/developer-center/issues/416) | heck82，2017-11-24。正文写 Deepin Desktop 15.5 beta，Arch。deepin 的 hualet 当天回复。控制中心里日期是 Friday, November 24, 2017 | [15:50:55](https://user-images.githubusercontent.com/17931180/33214102-2d62d9b2-d133-11e7-90cc-0287ce0034fc.jpg)、[15:51:26](https://user-images.githubusercontent.com/17931180/33214103-2d7efebc-d133-11e7-9cf5-5351ee4ec7b7.jpg)、[15:52:07](https://user-images.githubusercontent.com/17931180/33214104-2d9a93b6-d133-11e7-8d5d-09f384b06146.jpg)、[15:55:19](https://user-images.githubusercontent.com/17931180/33214105-2db52b54-d133-11e7-902f-26cac95f330e.jpg) | 浅色窗口。工具栏在标题栏里：侧栏按钮（打开时蓝底）、后退和前进合成一个描边组、计算机按钮（在“计算机”页是蓝底白显示器图标）、长地址槽、搜索在地址槽右端。主目录那张的视图按钮是图标\|列表两半，当前一半为蓝。侧栏顺序 Home、Desktop、Videos、Music、Pictures、Documents、Downloads、Trash，再是 Computer（整行浅蓝、蓝字）、磁盘、Computers in LAN。图标视图里选中的是名字上的实心蓝底白字，图标本身没有框。状态栏居中写 `8 items` 或 `1 folder selected (contains 0 item)`，右侧是蓝色滑块 |
| [Deepin Wiki：Deepin File Manager](https://wiki.deepin.org/en/Software/Offical_Project/Deepin_File_Manager) | 页面描述的是经典深度文件管理器的导航栏、地址栏、视图、设置和状态栏。正文写列表显示图标、名字、修改时间、大小和类型。这次没有把页面里的图下成功（抓取超时），所以只用这段文字，不用没看到的图 | — | 列表有类型列。交互步骤（新建、复制、视图切换）只作文字，不当成已验证的运行时行为 |
| [ORDINATECHNIC：Deepin 15.4 Review](https://www.ordinatechnic.com/distribution-reviews/Deepin/deepin-154-review) | 2017-05-01，Deepin 15.4。文中写文件管理器列表只有 Name、Time Modified、Size、Type | 文中配图没有一张被确认是文件管理器列表，因此没有拿来当布局依据 | 列名和 15.5 截图的窗口是同一代产品；列顺序来自文字，不是来自一张列表截图 |

没有采用的图：`https://www.deepin.org/zh/original/dde-file-manager/` 上的四张预览是 2024-05 上传的（`计算机-无投影.png` 等），是现行文件管理器的宣传图，不是 Deepin 15。

## 2026-11-21，从合并后的 main 继续

标题栏高 40 px，和列表头、状态栏的 27 px 分开。客户区装饰（`WindowDecorations::Client`）把最小化、最大化、关闭画在同一条栏的右侧。后退/前进/向上仍在一组里，向上保留；计算机按钮在这组右边，打开 `/`，不是深度那种分节的“计算机”页。没有 `user-dirs.dirs` 时，家目录里存在的 Desktop、Videos、Music、Pictures、Documents、Downloads 会进侧栏，回收站后面还有计算机，打开 `/`。根路径的面包屑写成 Computer。新安装的列表从左到右是 Name、Time changed、Size、Type。裸机调色板是浅色；主题文件仍优先。

## 按这些截图改过的，以及还没对齐的

上一轮（已合并）在真窗口里核对过的：

- 搜索按钮放进地址槽右端（网格视图仍把这个位置留给筛选和排序）。
- 图标视图的选中态改成文件名上的实心强调色底、白字，不再给整个图标铺一块选中框。窗口不活跃时这块底是淡蓝、字是前景色。
- 新安装的列表带类型列。这一轮把英文表头改成 Name / Time changed / Size / Type，并从左到右按这个顺序排。逻辑列名仍是 name、size、date、kind。
- 视图按钮：Ctrl+1 是列表，Ctrl+3 是图标视图。

2026-11-21 在 1100×700 的 Xvfb 窗口里又核对过：客户区装饰下只有一条 40 px 栏，上面是侧栏、后退、前进、向上、计算机、地址槽、搜索，右侧是视图按钮和最小化、最大化、关闭。点计算机进入 `/`，按钮呈选中。没有 `user-dirs.dirs` 时侧栏是 Home、Desktop、Videos、Music、Pictures、Documents、Downloads、Trash，回收站后面是 Computer。点 Computer 后地址槽写成 Computer，列表是 `/`。在别的目录里这一行不保持选中，根磁盘行仍按原来的规则点亮。窗口获得 X 焦点后，图标视图里点中的 `media` 文件名是实心 `#2ca7f8`、白字，图标没有选中框。列表里点中的行是整行浅蓝，类型列写 Folder。

2026-11-21 那一列里「计算机打开 `/`」已经过时，下面按 2026-11-23 的窗口重测结果改写。

## 2026-11-23 计算机页和标题栏

`computer://` 是导航历史里的一个地点，不是把 `/` 改名。后端对这个地点回一份空列表并停掉目录监视，窗口再按真实的用户目录和已挂载卷画分节页。侧栏和工具栏的计算机都打开它。地址栏提交「Computer」或 `computer://` 也回到这一页。`/` 的面包屑不再写成 Computer。

在 900×600 的 Xvfb + Openbox 窗口里用鼠标和键盘核对过的：

- 点侧栏 Computer、点工具栏计算机、地址栏输入 Computer 后回车，主区是「My Directories」和「Internal Disk」，没有 Name / Time changed / Size / Type 表头。六个存在的用户目录用蓝色文件夹，上面是自制的小标记（显示器、播放、音符、圆点、横线、向下箭头）。根卷显示 System Disk，侧栏同一行也是 System Disk，不再用磁盘标签原文。
- 单击用户目录后，状态栏写 “1 folder selected (contains N item/items)”。N 是这个目录里不以点开头的条目数：空的 Music 是 0 items，只有 alpha.txt 的 Desktop 是 1 item。点系统盘仍是原来的 “1 of 7 selected”，没有假装它是文件夹。没选中时是 “7 items”。双击或方向键后回车打开真实目录。后退回到计算机页。向上在这一页是灰的。在家目录点向上进入 `/tmp`，再后退回到家目录，再前进回到 `/tmp`。
- 侧栏悬停：新安装的「Highlight on hover」默认关着，这是已经合并的设置。把它打开后，指针停在 Music 上，这一行是灰色，当前的 Home 仍是浅蓝。
- 最大化按钮来回切换：900×600 和铺满 1100×679。关闭按钮会关掉窗口。双击标题栏上没有控件的位置（按钮和地址槽之间的上沿，以及地址槽下沿的空白）同样来回切换这两档。客户区右下角向内拖，窗口从 900×600 变成 843×543。
- 图标视图、列表视图、Ctrl+L 路径、Ctrl+1 列表、Shift 连选、方向键把 `/usr/bin` 滚到新的一段、右键菜单、属性对话框（路径、类型、大小、时间、权限、所有者）、搜索 `alpha`（`/usr/bin` 里 1 条）、复制、粘贴、F2 改名、Delete 进回收站，都作用在真实文件上。回收站里能看到被移走的夹具文件。

## 对照状态

已完成并且在真窗口里测过：

- 计算机分节页、侧栏高亮、地址标签、后退、双击和键盘打开真实路径。
- 计算机页不画列表表头，状态栏右侧不再写 unknown，缩放滑块还在。选中一个用户目录时，状态栏是 “1 folder selected (contains N item/items)”，N 来自该目录。
- 标题栏双击空白最大化，客户区角落拖拽改变大小，最大化按钮和关闭按钮可用。
- 浅色选中的工具栏按钮是实色强调色、白色字形。
- 列表列顺序 Name / Time changed / Size / Type。裸机浅色调色板；主题文件仍优先。
- 复制、粘贴、改名、移到回收站是真实文件操作。

已接上，但这台 Openbox 没有跟着动：

- 标题栏按下并拖过 4 像素会调用 `start_window_move`。同一套按下再拖，在客户区角落能改大小，拖标题栏空白时窗口左上角仍停在原来的位置。无边框移动要等窗口管理器接受按下之后才发出的 `_NET_WM_MOVERESIZE`。

被环境挡住：

- GPUI 只在合成器同时声明 `_GTK_FRAME_EXTENTS` 时才用客户区装饰。这台 Xvfb 的 Openbox 没有这个原子，所以窗口外面仍有一条 Openbox 标题栏，客户区里再画一套最小化、最大化、关闭。不是用残缺的 GXDE 运行时当对照。
- 英文 0 用复数 “0 items”。15.5 那张截图的读数是 “0 item”。数量来自真实目录，没有为了凑那一个词改复数规则。

还能做、这次没做完：

- 面包屑分隔仍是 `/`，当前段是小蓝块，不是地址槽里的一整段文字。
- 视图按钮仍是列表、分栏、图标、双栏四个。
- 文件夹标记是自制的单色图形，不是深度那套彩色图标。GPL 资源没有并入。
- 右键菜单沿用已有项，没有加没有后端的 Display as、Compress、Send to desktop。
- 状态栏仍是 27 px，普通目录仍显示剩余空间。
