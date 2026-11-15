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
- 没有在本环境里跑起 gxde-file-manager，因此没有并排截图。对照来自上面的主题和布局源码。
- 无头 GPUI 截到了深色列表、深色图标视图和浅色列表（顶栏、面包屑当前段、列头、状态栏和缩放滑块）。测试夹具没有画出侧栏地点，所以侧栏行没有出现在这些图里。截图像素缓冲区是 2 倍，图是裁到实际 1100×700 窗口后的结果。
