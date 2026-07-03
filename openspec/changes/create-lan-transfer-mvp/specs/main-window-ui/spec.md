## ADDED Requirements

### Requirement: 提供单主页面工作台

系统 MUST 在 v0.1 中使用单主页面结构承载本机状态、目标连接、文件发送、接收请求和传输任务。

#### Scenario: 打开主窗口
- **WHEN** 用户启动应用
- **THEN** 系统 MUST 显示局域网文件传输主页面

#### Scenario: 三栏布局
- **WHEN** 主窗口宽度足以展示桌面布局
- **THEN** 系统 MUST 按 UI 参考图呈现左侧本机状态、中间连接与发送、右侧传输任务的工作台布局

### Requirement: 展示本机状态区域

系统 MUST 在主页面左侧或等价区域展示设备名称、访问地址、服务状态、网络候选和保存目录。

#### Scenario: 本机状态完整展示
- **WHEN** 本地服务启动完成
- **THEN** 系统 MUST 展示设备名称、访问地址、服务运行状态和保存目录

#### Scenario: 网络候选展示
- **WHEN** 系统发现多个候选 IPv4 地址
- **THEN** 系统 MUST 展示候选地址列表并标识推荐地址

### Requirement: 展示目标连接区域

系统 MUST 在主页面提供目标设备连接输入、连接操作和最近连接列表。

#### Scenario: 输入目标地址
- **WHEN** 用户进入目标设备连接区域
- **THEN** 系统 MUST 提供接收方地址输入框和连接测试操作

#### Scenario: 展示最近连接
- **WHEN** 存在最近连接设备
- **THEN** 系统 MUST 展示设备名、地址、最近连接状态或时间

### Requirement: 展示文件发送区域

系统 MUST 在主页面提供拖拽区、选择文件操作、待发送列表和发送按钮。

#### Scenario: 空待发送列表
- **WHEN** 用户尚未选择文件
- **THEN** 系统 MUST 展示拖拽文件到此处和点击选择文件的入口

#### Scenario: 有待发送文件
- **WHEN** 待发送列表不为空
- **THEN** 系统 MUST 展示待发送文件列表和发送文件按钮

### Requirement: 展示接收确认弹窗或确认区

系统 MUST 在收到传输请求时展示清晰的接收确认界面，样式参考 `UI/confirm.png`。

#### Scenario: 收到接收请求
- **WHEN** 接收方收到新的有效传输请求
- **THEN** 系统 MUST 展示接收文件请求界面，并提供拒绝和接收操作

#### Scenario: 展示文件明细
- **WHEN** 接收确认界面打开
- **THEN** 系统 MUST 展示发送方、地址、文件数量、总大小和文件明细

### Requirement: 展示传输任务区域

系统 MUST 在主页面展示传输任务列表，并支持按全部、发送、接收、已完成等视图筛选。

#### Scenario: 展示任务条目
- **WHEN** 存在传输任务
- **THEN** 系统 MUST 展示文件名、大小、方向、进度、状态和可用操作

#### Scenario: 筛选任务
- **WHEN** 用户切换任务筛选标签
- **THEN** 系统 MUST 按所选范围展示任务

#### Scenario: 无任务
- **WHEN** 当前筛选范围没有任务
- **THEN** 系统 MUST 展示空状态

### Requirement: 保持 UI 参考约束

系统 MUST 以 `UI/design.png` 和 `UI/confirm.png` 作为 v0.1 视觉与信息架构参考，同时以 PRD 功能要求为最终范围。

#### Scenario: 主页面信息架构
- **WHEN** 实现主页面
- **THEN** 系统 MUST 覆盖 UI 参考图中的本机状态、目标设备连接、发送文件和传输任务区域

#### Scenario: 接收确认信息架构
- **WHEN** 实现接收确认
- **THEN** 系统 MUST 覆盖 UI 参考图中的标题、发送方、文件列表、拒绝和接收操作
