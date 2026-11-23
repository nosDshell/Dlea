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
| 按钮尺寸 | 24×24，圆角 4 | 默认字号下命中盒宽 24。标题栏高 40。地址井比栏矮 10，圆角 4 |
| 深色按钮 | 静止白 15%、悬停 30%、按下 10%、禁用 6%；选中为实色强调色 | 同 alpha。选中图标仍用主题前景色，以保持原有标记对比度测试 |
| 浅色按钮 | 白底、10% 描边，悬停为蓝色渐变 | 白底和描边。悬停是强调色 16% 淡洗，而不是 `#8CCFFF→#4BB8FF` 渐变，这样深色图标仍可读 |
| 面包屑 | 深色井 `rgba(255,255,255,0.15)`，浅色白底描边，当前段实色强调色、白字、圆角 4 | 同井。分隔是 `›`，段之间 4 px。当前段是实色强调色、浅色字，只在右侧圆角 4。字号仍是 caption |
| 视图托盘 | `rgba(0,0,0,0.1)`，内边距 1，按钮贴齐 | 浅色和深色都是 10% 黑底，按钮贴齐。当前格是实色强调色、浅色字形。图标、列表、`▾` 仍在这一枚里。分栏和双栏在 `▾` 里 |
| 侧栏行 | 高 30、图标 16、左边距 13；选中 `#332ca7f8`，悬停 `#1Affffff` | 已按这组数字和 alpha |
| 列表选中 | 强调色 76/255 填充、150/255 描边 | 列表圆角 2，图标视图圆角 3 |
| 列头 | `QHeaderView` 黑 10% | 同 |
| 状态栏 | 项数居中，右侧 120 px 缩放滑块、右边距 20 | 项数居中，滑块保留。磁盘余量不再画出；`statusFooterState` 仍报告文件系统 |
| 标签页 | 方角、约 8% 底 | 沿用已落地的 GXDE 标签底色 |
| 菜单 | 强调色 20% 高亮，圆角约 6 | 沿用已落地的菜单高亮 |

## 未对齐

- 窗口装饰不是 DTK `DTitlebar`。最小化、最大化和关闭由 GPUI 画在客户区标题栏里。关闭在悬停时是红色底、浅色叉。
- 图标不是 `themes/dark/icons` 里的那套彩色侧栏和工具栏 SVG。那些资源是 GPL-3.0，没有并入 MIT 的 Dlea。
- 浅色悬停不是 QSS 里的蓝渐变。
- 面包屑字号仍是 caption，大约是正文字号的 0.833。当前段已经是只在右侧圆角的实色块。
- 分栏和双栏不在托盘的两个主按钮里，而在旁边的 `▾`。列表和分栏里搜索在地址槽右端；网格模式的筛选、排序仍在托盘左边。
- 状态栏不再画出磁盘余量。短通知留在居中项数和滑块之间；放不下时项数改到左边距 14。项数字体颜色是次级色。
- 计算机页的文件夹和磁盘仍是自制图形。外形已经是浅色折页文件夹和带底座、灯、栅的磁盘，再改像素已经看不出差别，也到不了深度那套图标。没有拷贝那些文件。
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

- 点侧栏 Computer、点工具栏计算机、地址栏输入 Computer 后回车，主区是「My Directories」和「Internal Disk」，没有 Name / Time changed / Size / Type 表头。900 px 宽时用户目录是第一行四个、第二行两个，和 15.5 那页的换行一样。点 Desktop 再按向下，选中落到 Documents。六个目录用蓝色文件夹，上面是自制的小标记。根卷显示 System Disk，侧栏同一行也是 System Disk。
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
- 列表列顺序 Name / Time changed / Size / Type。没画出来的列按这个顺序贴在右边邻居上。裸机浅色调色板；主题文件仍优先。
- 复制、粘贴、改名、移到回收站是真实文件操作。

已完成并且在真窗口里测过，补记：

- 标题栏按下并拖过 4 像素后，指针继续移动，窗口才跟着走。Openbox 上从 100,58 到 50,128。xfwm4 上从 0,0 到 80,146。只把整段位移放在窗口管理器接到 `_NET_WM_MOVERESIZE` 之前，窗口不会动。
- xfwm4 声明了 `_GTK_FRAME_EXTENTS`。这时 `_MOTIF_WM_HINTS` 是 `0x2, 0x0, 0x0, 0x0, 0x0`，窗口外面没有第二条标题栏，双击空白铺满 1100×700。Openbox 不声明这个原子，GPUI 就改用服务器装饰，于是外面再有一条标题栏。
- 计算机页的磁盘用主题的 `executable`（绿），可移动卷用标签色里的橙色，选中字和标记用 `background`。`tools/dlea-gpui-lint` 里这轮新增的字面颜色已经没有了。剩下 18 处和 `origin/main` 相同。
- 地址栏当前段是强调色文字，不再铺一块蓝底。`~/Desktop` 里点 `~` 回到家目录。

## 2026-11-23 状态栏、面包屑、视图和悬停

对照的是 GXDE `char/qt6_migration` 源码，不是那次缺插件的运行时。

- 状态栏内控件高度来自 `dstatusbar.cpp` 的 `STATUSBAR_WIDGET_DEFAULT_HEIGHT` 24，`sizeHint` 至少 22。列表头仍按字号算（正文字号 14 时是 27）。状态条高度是 `(列表头 * 24 / 27)` 四舍五入，所以默认字号是 24，字号变了它跟着变。`Geometry` 把列表头和状态条分开记。`composition.rs` 原来正好 401 行，绘制函数挪到 `composition_render.rs`，没有把记录上限撑破。1100×700、列表头 27 时，单栏内容高 609、带标签页时 582。
- 面包屑画出来的分隔是 `›`（U+203A），一个字符，和 GXDE `DFMCrumbBar` 按钮之间不放斜杠一致。编辑框、复制地址、每个面包屑指向的目录仍是带 `/` 的真实路径。`~/Work/claude` 画成 `~›Work›claude`，最后一段是强调色。
- 工具栏主视图是图标然后列表，和 15.5 图标视图那张的两段控件一致。分栏和双栏在旁边的 `▾` 菜单里，点下去发出原来的 `viewColumns` / `viewDual`。这个标记和设置按钮分开，避免两个相同的汉堡。Ctrl+1 到 Ctrl+4 仍切换四个视图。菜单用 `deferred` 画在栏的外面，点 Columns 后窗口变成分栏。
- 悬停：`DStyledItemDelegate::hoverBoxEnabled` 读 `GA_ShowHoverBox`，`gxde-file-manager.default.json` 里是 `true`。浅色侧栏悬停底是 `light.ini` 的 `#0C000000`，深色是 `dark.ini` 的 `#1Affffff`。Dlea 的新文档现在默认打开 Highlight on hover。文件里写了 `false` 的仍是关。已经存过、但没有这个键的旧文件在读取时保持关，空的新文档仍是开。缺这个键时洗色本身也不画。洗色仍是前景色的 10%，和深色主题那条 `#1Affffff` 一致；浅色那条 `#0C000000` 更淡，对比度门闩把 10% 钉死了，这次没有拆成两套透明度。新家目录里 Music 一行是灰的，当前的 Home 仍是浅蓝。

xfwm4 上仍是一条客户区标题栏，拖动、最大化、最小化可用。Openbox 不声明 `_GTK_FRAME_EXTENTS`，GPUI 就改用服务器装饰，窗口外面会再有一条标题栏。这是窗口管理器的能力，不是再画一条栏。

被环境挡住：

- 网络区没有接入发现后端。`device_rail` 里写明还没有 NETWORK producer，所以计算机页不画「局域网上的计算机」。已挂载的卷仍然来自真实的设备列表。
- `qs` 是 quickshell 的命令名。这个系统的 apt 没有 quickshell 包，没有装一个来历不明的二进制顶上。依赖 `qs` 的套件因此跑不了。
- `qml6` 这个命令名也不在已安装的 Qt 包里。`qmllint` 在 `/usr/lib/qt6/bin/qmllint`，`qmlcachegen` 在 `/usr/lib/qt6/libexec/qmlcachegen`，`plugins.qmltypes` 在 `/usr/lib/x86_64-linux-gnu/qt6/qml/QtQuick/`。`bubblewrap` 已安装，`bwrap` 可用。

不照抄：

- 英文 0 走目录里的复数规则，所以是 “0 items”。截图里的 “0 item” 不合这条规则。

还能做、这次没做完：

- 右键菜单沿用已有项，没有加没有后端的 Display as、Compress、Send to desktop。

## 2026-11-23 傍晚，计算机页图标和工具栏胶囊

`DISPLAY=:99`，xfwm4。客户区窗口 900×600，`_NET_FRAME_EXTENTS` 是 `0, 0, 0, 0`，`_MOTIF_WM_HINTS` 是 `0x2, 0x0, 0x0, 0x0, 0x0`。外面没有第二条标题栏。

- 计算机页的文件夹改成带折页的自制图形，标记是浅色，画在折页下面。磁盘改成扁的圆角矩形，带一条槽和一个点。不是深度那套 GPL 图标。900 px 宽仍是第一行四个、第二行两个。节标题右边有一条发丝线。点 Documents，名字是实心强调色、浅色字，状态栏是 `1 folder selected (contains 1 item)`。
- 图标和列表收进同一枚描边胶囊，`▾` 在胶囊里面。点开后菜单是 Grid / List / Columns / Dual pane，没有被胶囊裁掉。点 Columns 后，Documents 里是左栏 `notes.txt`、右栏预览。地址是 `~›Documents`，当前段是强调色。
- 浅色状态栏滑块的圆钮用背景色，所以是浅圆，不再是黑圆。轨道仍是强调色。
- 地址井高度是标题栏减 10。标题栏仍是 40。

`cargo test --offline --locked -p dlea-gui --lib` 里这三组通过：`visual_geometry` 9、`computer::` 4、`plate::tests` 2。

`./tests/run-all.sh` 已经结束，不是通过。395 套跑完，335 套失败，退出码 79。父进程 50585 已经退出。分类：

- 约 162 套缺 `qs`。没有安装来历不明的 quickshell。
- 约 56 套要 `/home/dlea-sandbox`，这里建不了。
- 约 13 套要名为 `qml6` 的命令。`qmllint` 和 `qmlcachegen` 在 Qt 的目录里，命令名对不上。
- 约 18 套是 `dlea-gui` 的 clippy `-D warnings`：`ground.rs` 参数过多、`paint.rs` 多余借用、`button()` 参数过多。这些在这次改动之前的记录里就已经在。
- `search-fb5` 的三个失败测试在 `src/backend/searchmatch_tests.rs`、`searchrank_tests.rs`、`searchtags_write_tests.rs`。本分支没有改这些搜索文件。
- `cli-contract` 仍在对 `flea` / `FLEA_`。没有整份改黄金文件。
- 日志里的 `state-compat` 失败是跑的过程中读到了正在改的树。结束后用当前的 `target/debug/dlea` 重跑 `tests/compat/state_compat.py`，退出码 0。
- 日志里 `g2-pv2-md` 和 `g2-pv2-web` 的编译错误也出在树还在改的时候。当前 `cargo build --offline --locked -p dlea-gui --bin dlea-gui` 通过。

没有把这次跑完称作整库验收通过。

## 2026-11-23 夜里，关闭按钮、空目录和状态栏

上游 `origin/main` 仍是 `d41392b9`。这一轮开始时本地和远程都是 `ab8577e02`，没有新提交，没有合并。

`DISPLAY=:99`，xfwm4，窗口 900×600。对照的是 2017-11-24 的 DDE 15 截图，不是缺插件的 GXDE 运行时。

- 关闭按钮悬停是红底。采样到的红是 rgb(223, 42, 42)，叉是接近白的 rgb(254, 252, 252)。静止时仍是主题前景色的叉，没有红底。
- 状态栏右边不再画出磁盘余量。项数在栏的中间，滑块在右边，浅色圆钮还在。文件系统字符串仍留在绘制探针里，所以 `statusFooterState.filesystem` 还在。状态条内部高度改成和槽一样的 24（字号 14 时），不再用 27 高的行高撑出槽外。
- 新文档的空目录默认关掉 Omi 和轮换，固定句改成 “Folder is empty”，不再整句大写。已经存过、但没有这两个键的旧文件在读取时仍保持 0.4.1 的开。轮换打开时，原来的三句提示还在。其它语言的既有译法没有整批改写。
- 计算机页的文件夹仍是自制的折页图形，正文用强调色，折页更深，标记是浅色。磁盘是更亮的同色相圆角块，底下留一条深色边和浅色格栅。没有拷贝深度的图标文件。从空目录再进计算机页时，空目录那句不再留在磁贴上。
- 没有局域网行。英文 0 仍是 “0 items”。

这几张是这次窗口上的图：列表、红色关闭、空目录、计算机页。显示效果还不是 15.5 的一比一。长通知和居中项数会叠在一起；`status_tests` 和 `status_notice_layout_tests` 仍按旧的左对齐 27 px 状态栏锁坐标，上一提交里就已经对不上 `0 673`，这次没有把它们改成通过。`status.rs` 仍超过记录的 408 行。`tests/compat/state_compat.py` 用当前 `target/debug/dlea` 重跑，退出码 0。没有重跑整库 `tests/run-all.sh`，上次的 395 套、335 套失败、退出码 79 仍然有效，不能当验收通过。

## 2026-11-23 夜里，通知让开项数

上游 `origin/main` 仍是 `d41392b9`。这一轮开始时本地和远程都是 `40284dfb0`。

状态栏仍是 DDE 15 那条：没有磁盘余量，滑块在右边。短通知放得进项数和滑块之间的空隙时，项数继续居中。句子比这空隙长时，项数改到左边距 14，通知用剩下的宽度，右缘停在滑块前面，不再叠在项数上。1100×700 的帧是 `0 676 1100 24`：状态槽 24，不是列表头的 27。空目录英雄从列表头下沿开始，所以是 `192 67 908 609`。非错误通知用前景色，项数和 “esc dismisses” 仍是次级色。

计算机页的文件夹仍是自制图形：浅色正文、更深的折页和底边，标记用强调色而不是白。采样到的正文约 rgb(137, 205, 250)，标记是 rgb(44, 167, 248)。磁盘是高的圆角块：亮绿面板、浅灰底座、一盏灯和四条栅。没有拷贝深度的图标文件。

`DISPLAY=:99`，xfwm4，窗口 900×600。列表的 “11 items” 居中。通知栏左边是 `unavailable`，右边是 `Computer could not be read. · esc dismisses`，中间没有叠字。空目录仍是 “Folder is empty”，项数是 “0 items”。截图是 `dlea-lane-list.png`、`dlea-lane-notice.png`、`dlea-lane-notice-bar.png`、`dlea-lane-computer.png`、`dlea-lane-empty.png`，图标放大是 `dlea-lane-folder-zoom.png` 和 `dlea-lane-disk-zoom.png`。

`cargo test --offline --locked -p dlea-gui --lib window::status`：76 通过。没有重跑整库 395 套。`status.rs` 是 526 行，记录上限仍是 408。显示效果还不是 15.5 的一比一。

## 2026-11-23 夜里，面包屑和视图托盘

上游 `origin/main` 仍是 `d41392b9`。这一轮开始时本地和远程都是 `2387831e5`，领先 12、落后 0。

文件夹和磁盘没有再改。上一轮的浅色正文、深色折页和标记，以及带底座的磁盘，已经是自制图形能做到的程度。参考里的那套是深度的彩色图标，没有搬进来。

- 地址栏当前段改成实色强调色块。左边是方的，右边圆角 4。字用浅色角色：浅色窗口用背景色，深色窗口用前景色，没有写死白色。前面的段仍是次级色。分隔仍是 `›`，段后面留 4 px，不再贴住下一个名字。字号仍是 caption。改成正文字号会把按 caption 格子算的点击整段挪走，参考截图上也没有量到更大的地址字号。
- 视图托盘不再是白底描边胶囊。浅色和深色都用 10% 黑底，图标、列表、`▾` 贴在里面。当前格是实色强调色、浅色字形。分栏和双栏仍在 `▾` 里。菜单里当前行同样是实色强调色、浅色字。
- `status.rs` 的绘制挪到 `status_render.rs`。`status.rs` 现在是 323 行，低于记录的 408。`status_render.rs` 是 222 行。文件预算不再因为这一份报 GREW。别的 GREW 是原来就有的，这次没动那些文件。

`DISPLAY=:99`，xfwm4，窗口 900×600。`~/Documents/Notes` 的地址是灰色的 `Documents ›`，后面一块蓝底浅色字的 `Notes`。视图托盘是灰底，列表格是蓝的、字形是浅的。菜单打开时 List 一行是蓝底浅色字。截图是 `dlea-crumb-list.png`、`dlea-crumb-zoom.png`、`dlea-view-zoom.png`、`dlea-view-menu.png`。

`cargo test --offline --locked -p dlea-gui --lib window::status`：76 通过。`g1_12` 里面和面包屑中心、宽名字点击有关的测试跟着新间距改了断言，通过。没有重跑整库 395 套。显示效果还不是 15.5 的一比一。
