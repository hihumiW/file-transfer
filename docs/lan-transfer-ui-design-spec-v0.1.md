# 局域网文件传输工具 UI 设计规范 v0.1

> 适用范围：桌面端局域网文件传输应用。  
> 技术方向：Tauri 2 + React + TypeScript + Tailwind CSS。  
> 设计目标：轻量、干净、克制、偏 macOS 工具软件气质，避免营销感和大面积插画。

---

## 1. 设计原则

### 1.1 产品气质

整体视觉应接近 macOS 工具软件，而不是 SaaS 官网或营销落地页。

关键词：

- 轻量
- 干净
- 克制
- 清晰
- 生产力工具
- 低打扰
- 状态明确

界面需要帮助用户快速判断：

- 本机服务是否正常运行；
- 当前局域网地址是什么；
- 是否已经连接目标设备；
- 当前选择了哪些文件；
- 是否有待确认的接收请求；
- 传输任务处于什么状态。

### 1.2 视觉克制原则

不建议使用：

- 大面积插画；
- 强营销文案；
- 高饱和大色块；
- 复杂渐变背景；
- 过强阴影；
- 过多装饰性元素。

建议使用：

- 中性色背景；
- 白色卡片；
- 细边框；
- 轻微阴影；
- 小面积状态色；
- 稳定的网格布局；
- 清晰的信息层级。

---

## 2. 页面结构

### 2.1 主界面结构

主界面采用单窗口三栏结构。

```text
┌────────────────────────────────────────────────────────────┐
│ macOS title bar / 应用标题 / 工具按钮                         │
├───────────────┬────────────────────────────┬───────────────┤
│ 本机状态       │ 目标设备连接                 │ 传输任务       │
│ 网络地址       ├────────────────────────────┤ 任务列表       │
│ 保存目录       │ 发送文件                     │               │
└───────────────┴────────────────────────────┴───────────────┘
```

主界面包含：

1. 本机状态区；
2. 目标设备连接区；
3. 文件拖拽发送区；
4. 待发送文件列表；
5. 传输任务列表。

接收确认不放在主界面 panel 中，而是作为独立弹窗出现。

### 2.2 接收请求弹窗

接收请求是一个需要用户立即决策的场景，应设计为独立弹窗。

弹窗内容包括：

- 请求来源设备；
- 来源 IP / 端口；
- 请求时间；
- 文件数量；
- 总文件大小；
- 文件列表；
- 拒绝按钮；
- 接收按钮。

---

## 3. 基础布局尺寸

### 3.1 主窗口尺寸

推荐默认尺寸：

```text
width: 1280px - 1440px
height: 760px - 860px
min-width: 960px
min-height: 680px
```

主窗口样式：

```css
.app-window {
  background: #F6F7F9;
  border: 1px solid #DDE2EA;
  border-radius: 14px;
  box-shadow: 0 20px 60px rgba(15, 23, 42, 0.16);
  overflow: hidden;
}
```

### 3.2 标题栏

```css
.window-titlebar {
  height: 48px;
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}
```

macOS 窗口按钮：

```text
button size: 12px
button gap: 8px
left offset: 20px
```

标题文字：

```css
.window-title {
  font-size: 15px;
  line-height: 22px;
  font-weight: 600;
  color: #111827;
}
```

右上角工具按钮：

```text
icon button size: 32px
gap: 8px
right offset: 20px
```

---

## 4. 栅格与自适应布局

### 4.1 默认三栏布局

主内容区推荐使用 CSS Grid。

```css
.main-layout {
  display: grid;
  grid-template-columns: 300px minmax(420px, 1fr) 380px;
  gap: 12px;
  padding: 0 20px 16px;
}
```

三栏职责：

| 区域 | 宽度 | 内容 |
| --- | ---: | --- |
| 左栏 | 300px | 本机状态、网络地址、保存目录 |
| 中栏 | 自适应 | 目标设备连接、发送文件 |
| 右栏 | 360px - 400px | 传输任务 |

### 4.2 宽屏布局：≥ 1280px

```css
.main-layout {
  grid-template-columns: 300px minmax(480px, 1fr) 380px;
  gap: 12px;
}
```

这是主设计图的默认状态。

### 4.3 中等宽度：1024px - 1279px

```css
.main-layout {
  grid-template-columns: 280px minmax(380px, 1fr) 340px;
  gap: 10px;
}
```

调整策略：

- 左栏略微收窄；
- 右侧任务列表减少横向 padding；
- 最近连接设备中的次级信息允许省略；
- 文件名使用省略号处理。

### 4.4 窄桌面：900px - 1023px

```css
.main-layout {
  grid-template-columns: 260px minmax(420px, 1fr);
  grid-template-areas:
    "sidebar connect"
    "sidebar send"
    "sidebar tasks";
}
```

此时右侧传输任务下移到中栏下方。

### 4.5 极窄窗口：< 900px

虽然是桌面端工具，不建议小于 900px，但仍需有基本自适应能力。

```css
.main-layout {
  grid-template-columns: 1fr;
}
```

处理策略：

- 本机状态变为顶部 summary card；
- 网络候选列表默认折叠；
- 目标设备连接、发送文件、传输任务纵向排列；
- 任务状态允许换行；
- 文件名、地址全部使用省略号。

---

## 5. 间距规范

设计使用 4px 基础栅格。

| Token | 数值 | 用途 |
| --- | ---: | --- |
| `space-1` | 4px | 极小间距、图标与文字 |
| `space-2` | 8px | 小组件内部间距 |
| `space-3` | 12px | 列表项、卡片间距 |
| `space-4` | 16px | panel padding |
| `space-5` | 20px | 窗口内容边距 |
| `space-6` | 24px | 弹窗 padding、大区块间距 |
| `space-8` | 32px | 大间距 |

### 5.1 页面级 spacing

```text
窗口内容 padding: 20px
三栏 gap: 12px
panel gap: 12px
底部状态栏高度: 32px
```

### 5.2 Panel 内部 spacing

```text
panel padding: 16px
panel title margin-bottom: 14px
section gap: 12px
list item padding: 10px 12px / 12px 14px
```

### 5.3 表单控件 spacing

```text
input height: 40px
button height: 40px
small button height: 32px
icon button size: 32px
input horizontal padding: 12px
button horizontal padding: 16px - 18px
```

---

## 6. 字体规范

### 6.1 字体族

优先使用系统字体。

```css
:root {
  font-family:
    -apple-system,
    BlinkMacSystemFont,
    "SF Pro Text",
    "SF Pro Display",
    "PingFang SC",
    "Microsoft YaHei",
    system-ui,
    sans-serif;
}
```

### 6.2 字号层级

| 用途 | 字号 | 字重 | 行高 | 示例 |
| --- | ---: | ---: | ---: | --- |
| 窗口标题 | 15px | 600 | 22px | 局域网文件传输 |
| Panel 标题 | 15px | 600 | 22px | 本机状态 / 发送文件 |
| 主信息标题 | 14px | 600 | 20px | ThinkBook |
| 正文 | 13px | 400 / 500 | 20px | 192.168.1.23:7788 |
| 次级说明 | 12px | 400 | 18px | 上次连接：昨天 15:30 |
| 状态标签 | 12px | 600 | 16px | 发送完成 |
| 小型辅助文字 | 11px | 400 | 16px | 上传：12.4 MB/s |
| 弹窗标题 | 20px | 600 | 28px | 接收文件请求 |

### 6.3 文本省略

文件名、设备名、IP 地址等都需要支持省略号。

```css
.text-ellipsis {
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}
```

---

## 7. 颜色规范

### 7.1 中性色

```css
:root {
  --bg-app: #F6F7F9;
  --bg-panel: #FFFFFF;
  --bg-subtle: #F8FAFC;
  --bg-hover: #F1F5F9;
  --bg-selected: #EAF2FF;

  --border-light: #E5EAF0;
  --border-default: #D8DEE8;
  --border-strong: #CBD5E1;

  --text-primary: #111827;
  --text-secondary: #475569;
  --text-tertiary: #64748B;
  --text-muted: #94A3B8;
}
```

### 7.2 品牌蓝色

```css
:root {
  --blue-50: #EFF6FF;
  --blue-100: #DBEAFE;
  --blue-500: #2563EB;
  --blue-600: #1D4ED8;
  --blue-700: #1E40AF;
}
```

使用场景：

- 主按钮；
- 当前连接状态；
- 推荐 IP；
- 链接文字；
- active tab；
- 拖拽上传图标。

### 7.3 状态色

状态色需要克制，避免刺眼。推荐使用浅色背景 + 深色文字 + 小图标。

```css
:root {
  --success-bg: #ECFDF3;
  --success-text: #15803D;
  --success-border: #BBF7D0;

  --warning-bg: #FFF7ED;
  --warning-text: #C2410C;
  --warning-border: #FED7AA;

  --danger-bg: #FEF2F2;
  --danger-text: #DC2626;
  --danger-border: #FECACA;

  --neutral-bg: #F1F5F9;
  --neutral-text: #475569;
  --neutral-border: #E2E8F0;
}
```

状态映射：

| 状态 | 背景 | 文字 | 图标 |
| --- | --- | --- | --- |
| 等待确认 | `--warning-bg` | `--warning-text` | clock |
| 传输中 | `--blue-50` | `--blue-500` | progress / upload |
| 发送完成 | `--success-bg` | `--success-text` | check |
| 接收完成 | `--success-bg` | `--success-text` | check |
| 发送失败 | `--danger-bg` | `--danger-text` | x / alert |
| 接收失败 | `--danger-bg` | `--danger-text` | x / alert |
| 已拒绝 | `--neutral-bg` | `--neutral-text` | minus |

---

## 8. Panel 规范

### 8.1 Panel 容器

```css
.panel {
  background: #FFFFFF;
  border: 1px solid #E5EAF0;
  border-radius: 10px;
  padding: 16px;
  box-shadow: 0 1px 2px rgba(15, 23, 42, 0.04);
}
```

### 8.2 Panel 标题

所有 panel 标题统一使用普通标题样式。

```css
.panel-title {
  font-size: 15px;
  line-height: 22px;
  font-weight: 600;
  color: #111827;
  margin-bottom: 14px;
}
```

注意：

- `发送文件` 是 panel 标题，不是 tab；
- 不加蓝色下划线；
- 不使用 active tab 背景；
- 与 `本机状态`、`目标设备连接`、`传输任务` 保持一致。

### 8.3 Panel 分割线

```css
.panel-divider {
  height: 1px;
  background: #E5EAF0;
  margin: 16px 0;
}
```

---

## 9. 按钮规范

### 9.1 主按钮

用于：连接、发送文件、接收。

```css
.button-primary {
  height: 40px;
  padding: 0 18px;
  border-radius: 8px;
  border: none;
  background: #2563EB;
  color: #FFFFFF;
  font-size: 13px;
  font-weight: 600;
  box-shadow: 0 1px 2px rgba(37, 99, 235, 0.24);
  cursor: pointer;
}

.button-primary:hover {
  background: #1D4ED8;
}

.button-primary:active {
  background: #1E40AF;
}

.button-primary:disabled {
  background: #CBD5E1;
  color: #FFFFFF;
  box-shadow: none;
  cursor: not-allowed;
}
```

### 9.2 次按钮

用于：拒绝、取消、打开目录、选择目录。

```css
.button-secondary {
  height: 40px;
  padding: 0 16px;
  border-radius: 8px;
  border: 1px solid #D8DEE8;
  background: #FFFFFF;
  color: #111827;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
}

.button-secondary:hover {
  background: #F8FAFC;
}

.button-secondary:active {
  background: #F1F5F9;
}
```

### 9.3 文本按钮

用于：打开保存目录、点击选择文件、清空已完成。

```css
.button-link {
  padding: 0;
  border: none;
  background: transparent;
  color: #2563EB;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
}

.button-link:hover {
  color: #1D4ED8;
}
```

### 9.4 图标按钮

用于：复制地址、设置、更多操作、删除待发送文件。

```css
.icon-button {
  width: 32px;
  height: 32px;
  border-radius: 7px;
  border: none;
  background: transparent;
  color: #64748B;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
}

.icon-button:hover {
  background: #F1F5F9;
  color: #111827;
}
```

---

## 10. 输入框规范

### 10.1 普通输入框

```css
.input {
  height: 40px;
  border-radius: 8px;
  border: 1px solid #D8DEE8;
  background: #FFFFFF;
  padding: 0 12px;
  font-size: 13px;
  color: #111827;
  outline: none;
}

.input::placeholder {
  color: #94A3B8;
}

.input:focus {
  border-color: #2563EB;
  box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.12);
}

.input.error {
  border-color: #DC2626;
  box-shadow: 0 0 0 3px rgba(220, 38, 38, 0.10);
}
```

### 10.2 地址输入组合

目标设备连接输入区推荐结构：

```text
[ 输入对方 IP:端口，例如 192.168.1.8:7788 ] [连接] [下拉]
```

布局：

```css
.connect-form {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto auto;
  gap: 8px;
}
```

---

## 11. Tabs 规范

Tabs 只用于真正的分类切换，例如传输任务：

```text
全部 / 发送 / 接收 / 已完成
```

不要用于 `发送文件` panel 标题。

### 11.1 Tabs 容器

```css
.tabs {
  display: inline-flex;
  gap: 4px;
  padding: 3px;
  background: transparent;
}
```

### 11.2 Tab item

```css
.tab {
  height: 28px;
  padding: 0 12px;
  border-radius: 7px;
  border: none;
  background: transparent;
  color: #64748B;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
}

.tab:hover {
  background: #F1F5F9;
  color: #334155;
}

.tab.active {
  background: #EAF2FF;
  color: #2563EB;
  font-weight: 600;
}
```

---

## 12. 本机状态区规范

### 12.1 信息行

```css
.info-row {
  display: grid;
  grid-template-columns: 80px minmax(0, 1fr) auto;
  align-items: center;
  gap: 8px;
  min-height: 44px;
}
```

左侧 label：

```css
.info-label {
  font-size: 13px;
  color: #64748B;
}
```

右侧 value：

```css
.info-value {
  font-size: 13px;
  color: #111827;
  font-weight: 500;
}
```

### 12.2 状态点

```css
.status-dot {
  width: 7px;
  height: 7px;
  border-radius: 999px;
  flex: none;
}

.status-dot.connected {
  background: #22C55E;
}

.status-dot.warning {
  background: #F97316;
}

.status-dot.error {
  background: #EF4444;
}

.status-dot.blue {
  background: #2563EB;
}
```

---

## 13. 网络候选列表规范

```css
.network-list {
  border: 1px solid #E5EAF0;
  border-radius: 8px;
  overflow: hidden;
  background: #FFFFFF;
}

.network-item {
  min-height: 44px;
  padding: 0 10px;
  display: grid;
  grid-template-columns: 20px minmax(0, 1fr) auto;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: #111827;
}

.network-item.active {
  background: #EAF2FF;
  color: #2563EB;
}
```

推荐 IP 文案：

```text
192.168.1.23（推荐）
```

其他候选：

```text
192.168.31.5（有线）
10.8.0.14（VPN）
172.17.0.1（Docker）
```

---

## 14. 最近连接设备列表规范

### 14.1 列表容器

```css
.recent-device-list {
  border: 1px solid #E5EAF0;
  border-radius: 8px;
  overflow: hidden;
  background: #FFFFFF;
}
```

### 14.2 列表项

```css
.recent-device-item {
  min-height: 56px;
  padding: 10px 12px;
  display: grid;
  grid-template-columns: 28px minmax(0, 1fr) auto;
  gap: 10px;
  align-items: center;
  border-bottom: 1px solid #EEF2F7;
}

.recent-device-item:last-child {
  border-bottom: none;
}
```

设备名：

```css
.device-name {
  font-size: 14px;
  line-height: 20px;
  font-weight: 500;
  color: #111827;
}
```

设备地址：

```css
.device-address {
  font-size: 12px;
  line-height: 18px;
  color: #64748B;
}
```

已连接状态：

```css
.device-connected {
  color: #16A34A;
  font-size: 12px;
  font-weight: 600;
}
```

---

## 15. 文件拖拽发送区规范

### 15.1 Dropzone

```css
.dropzone {
  min-height: 120px;
  border: 1.5px dashed #CBD5E1;
  border-radius: 10px;
  background: #F8FAFC;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
  text-align: center;
}

.dropzone:hover,
.dropzone.drag-over {
  border-color: #2563EB;
  background: #EFF6FF;
}
```

图标：

```css
.dropzone-icon {
  width: 36px;
  height: 36px;
  color: #2563EB;
  margin-bottom: 8px;
}
```

主文案：

```css
.dropzone-title {
  font-size: 13px;
  line-height: 20px;
  color: #475569;
}

.dropzone-title .link {
  color: #2563EB;
  font-weight: 500;
}
```

辅助文案：

```css
.dropzone-desc {
  font-size: 12px;
  line-height: 18px;
  color: #64748B;
  margin-top: 2px;
}
```

---

## 16. 待发送文件列表规范

```css
.pending-file-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.pending-file-item {
  min-height: 36px;
  padding: 0 10px;
  border: 1px solid #E5EAF0;
  border-radius: 7px;
  display: grid;
  grid-template-columns: 20px minmax(0, 1fr) auto 28px;
  align-items: center;
  gap: 8px;
  background: #FFFFFF;
}
```

文件名：

```css
.pending-file-name {
  font-size: 13px;
  color: #111827;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}
```

文件大小：

```css
.pending-file-size {
  font-size: 12px;
  color: #64748B;
}
```

底部发送按钮靠右：

```css
.send-action-row {
  display: flex;
  justify-content: flex-end;
  margin-top: 12px;
}
```

---

## 17. 传输任务列表规范

### 17.1 任务列表容器

```css
.transfer-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
```

### 17.2 任务项

```css
.transfer-item {
  min-height: 58px;
  padding: 10px 12px;
  border: 1px solid #E5EAF0;
  border-radius: 8px;
  background: #FFFFFF;
  display: grid;
  grid-template-columns: 24px minmax(0, 1fr) auto;
  gap: 10px;
  align-items: center;
}

.transfer-item:hover {
  background: #F8FAFC;
}
```

文件名：

```css
.transfer-file-name {
  font-size: 13px;
  line-height: 20px;
  font-weight: 500;
  color: #111827;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}
```

文件信息：

```css
.transfer-file-meta {
  font-size: 12px;
  line-height: 18px;
  color: #64748B;
}
```

### 17.3 状态胶囊

状态颜色要柔和，不使用刺眼纯色文字裸露展示。

```css
.status-pill {
  height: 24px;
  padding: 0 8px;
  border-radius: 999px;
  font-size: 12px;
  font-weight: 600;
  display: inline-flex;
  align-items: center;
  gap: 4px;
  white-space: nowrap;
}
```

等待确认：

```css
.status-pill.pending {
  background: #FFF7ED;
  color: #C2410C;
}
```

传输中：

```css
.status-pill.uploading {
  background: #EFF6FF;
  color: #2563EB;
}
```

成功：

```css
.status-pill.success {
  background: #ECFDF3;
  color: #15803D;
}
```

失败：

```css
.status-pill.failed {
  background: #FEF2F2;
  color: #DC2626;
}
```

已拒绝：

```css
.status-pill.rejected {
  background: #F1F5F9;
  color: #475569;
}
```

推荐状态文案：

| 状态 | 文案 |
| --- | --- |
| `pending` | 等待确认 |
| `uploading` | 传输中 42% |
| `completed + send` | 发送完成 |
| `completed + receive` | 接收完成 |
| `failed + send` | 发送失败 |
| `failed + receive` | 接收失败 |
| `rejected` | 已拒绝 |

---

## 18. 进度条规范

大文件传输中可以在任务项下方展示细进度条。

```css
.progress-bar {
  height: 4px;
  border-radius: 999px;
  background: #E5EAF0;
  overflow: hidden;
  margin-top: 6px;
}

.progress-bar-fill {
  height: 100%;
  border-radius: inherit;
  background: #2563EB;
}
```

进度条使用规则：

- 仅传输中任务展示；
- 小文件可只展示百分比；
- 大文件建议展示百分比 + 速度；
- 成功或失败后隐藏进度条，仅保留状态胶囊。

---

## 19. 接收文件请求弹窗规范

### 19.1 弹窗尺寸

```text
width: 520px
min-height: 360px
max-height: 70vh
```

### 19.2 弹窗遮罩

```css
.modal-backdrop {
  position: fixed;
  inset: 0;
  background: rgba(248, 250, 252, 0.72);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
}
```

### 19.3 弹窗容器

```css
.modal {
  width: 520px;
  max-height: 70vh;
  background: #FFFFFF;
  border: 1px solid #E5EAF0;
  border-radius: 16px;
  box-shadow:
    0 24px 80px rgba(15, 23, 42, 0.20),
    0 4px 12px rgba(15, 23, 42, 0.08);
  padding: 24px;
}
```

### 19.4 弹窗标题

```css
.modal-title {
  font-size: 20px;
  line-height: 28px;
  font-weight: 600;
  color: #111827;
}
```

### 19.5 来源设备信息

```css
.request-device {
  display: grid;
  grid-template-columns: 32px minmax(0, 1fr) auto;
  gap: 12px;
  align-items: start;
  margin-top: 24px;
}
```

设备名：

```css
.request-device-name {
  font-size: 15px;
  line-height: 22px;
  font-weight: 600;
  color: #111827;
}
```

说明文字：

```css
.request-desc {
  font-size: 13px;
  line-height: 20px;
  color: #64748B;
}
```

### 19.6 文件列表

```css
.request-file-list {
  margin-top: 20px;
  border-top: 1px solid #E5EAF0;
  border-bottom: 1px solid #E5EAF0;
}

.request-file-item {
  min-height: 64px;
  display: grid;
  grid-template-columns: 28px minmax(0, 1fr) auto;
  gap: 12px;
  align-items: center;
  border-bottom: 1px solid #E5EAF0;
}

.request-file-item:last-child {
  border-bottom: none;
}
```

### 19.7 弹窗按钮区

```css
.modal-actions {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
  margin-top: 24px;
}

.modal-actions .button-primary,
.modal-actions .button-secondary {
  height: 44px;
  font-size: 14px;
}
```

按钮顺序：

```text
[拒绝] [接收]
```

---

## 20. 底部状态栏规范

底部状态栏用于展示当前连接状态与实时速度。

```css
.status-bar {
  height: 32px;
  padding: 0 20px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  color: #64748B;
  font-size: 12px;
}
```

左侧：

```text
● 已连接到 MacBook Pro (192.168.1.8:7788)
```

右侧：

```text
↑ 上传：12.4 MB/s    ↓ 下载：8.7 MB/s
```

---

## 21. Tailwind Token 建议

建议在 `tailwind.config.ts` 中定义基础 token。

```ts
import type { Config } from "tailwindcss";

const config: Config = {
  theme: {
    extend: {
      colors: {
        app: {
          bg: "#F6F7F9",
          panel: "#FFFFFF",
          subtle: "#F8FAFC",
          hover: "#F1F5F9",
          selected: "#EAF2FF",
        },
        line: {
          light: "#E5EAF0",
          DEFAULT: "#D8DEE8",
          strong: "#CBD5E1",
        },
        text: {
          primary: "#111827",
          secondary: "#475569",
          tertiary: "#64748B",
          muted: "#94A3B8",
        },
        brand: {
          50: "#EFF6FF",
          100: "#DBEAFE",
          500: "#2563EB",
          600: "#1D4ED8",
          700: "#1E40AF",
        },
        status: {
          successBg: "#ECFDF3",
          successText: "#15803D",
          successBorder: "#BBF7D0",
          warningBg: "#FFF7ED",
          warningText: "#C2410C",
          warningBorder: "#FED7AA",
          dangerBg: "#FEF2F2",
          dangerText: "#DC2626",
          dangerBorder: "#FECACA",
          neutralBg: "#F1F5F9",
          neutralText: "#475569",
          neutralBorder: "#E2E8F0",
        },
      },
      borderRadius: {
        panel: "10px",
        control: "8px",
        window: "14px",
        modal: "16px",
      },
      boxShadow: {
        panel: "0 1px 2px rgba(15, 23, 42, 0.04)",
        window: "0 20px 60px rgba(15, 23, 42, 0.16)",
        modal:
          "0 24px 80px rgba(15, 23, 42, 0.20), 0 4px 12px rgba(15, 23, 42, 0.08)",
      },
      fontSize: {
        ui11: ["11px", "16px"],
        ui12: ["12px", "18px"],
        ui13: ["13px", "20px"],
        ui14: ["14px", "20px"],
        ui15: ["15px", "22px"],
        modalTitle: ["20px", "28px"],
      },
    },
  },
};

export default config;
```

---

## 22. 推荐组件拆分

React 组件建议拆分如下：

```text
AppWindow
├── WindowTitlebar
├── MainLayout
│   ├── LocalStatusPanel
│   │   ├── DeviceInfo
│   │   ├── NetworkSelector
│   │   └── SaveDirectory
│   ├── CenterColumn
│   │   ├── TargetConnectionPanel
│   │   └── SendFilesPanel
│   │       ├── FileDropzone
│   │       ├── PendingFileList
│   │       └── SendAction
│   └── TransferTasksPanel
│       ├── TransferTabs
│       └── TransferTaskList
├── StatusBar
└── ReceiveRequestModal
```

---

## 23. 关键落地规则

1. 主界面只承载常驻信息和常规操作。
2. 接收请求使用独立弹窗，不占用主界面 panel。
3. `发送文件` 是普通 panel 标题，不做 tab 样式。
4. 只有 `传输任务` 的分类筛选使用 tabs。
5. 任务状态使用浅色胶囊，避免高饱和裸色文字。
6. 蓝色只用于连接、主按钮、链接和选中状态。
7. 绿色只用于成功，橙色只用于等待确认，红色只用于失败。
8. 三栏布局在宽屏下并排；中窄屏下传输任务下移。
9. 文件名、设备名、IP 地址必须支持省略号。
10. 主操作区域永远优先保证 `目标连接` 和 `发送文件` 的可用性。
