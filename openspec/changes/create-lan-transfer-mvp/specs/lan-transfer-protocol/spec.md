## ADDED Requirements

### Requirement: 提交传输请求

系统 MUST 在上传文件内容前，由发送方调用接收方 `POST /api/transfer/request` 提交传输元数据。

#### Scenario: 传输请求成功创建
- **WHEN** 接收方当前空闲且请求元数据有效
- **THEN** 接收方系统 MUST 创建待确认任务、生成 UUID v4 `transferId` 并返回 `status=pending`

#### Scenario: 接收方忙碌
- **WHEN** 接收方已有待确认或传输中的任务
- **THEN** 接收方系统 MUST 返回 `receiver_busy` 错误

#### Scenario: 发送方收到忙碌
- **WHEN** 发送方收到 `receiver_busy` 错误
- **THEN** 发送方系统 MUST 将任务标记为失败并提示对方正在处理其他传输任务

### Requirement: 接收确认状态轮询

系统 MUST 由发送方轮询 `GET /api/transfer/status/:transferId` 等待接收方确认结果。

#### Scenario: 等待确认
- **WHEN** 接收方尚未接收或拒绝任务
- **THEN** 状态接口 MUST 返回 `status=pending`

#### Scenario: 接收方已接收
- **WHEN** 接收方用户确认接收任务
- **THEN** 状态接口 MUST 返回 `status=accepted`

#### Scenario: 接收方已拒绝
- **WHEN** 接收方用户拒绝任务
- **THEN** 状态接口 MUST 返回 `status=rejected`

#### Scenario: 发送方轮询间隔
- **WHEN** 发送方等待接收方确认
- **THEN** 系统 MUST 以约 1 秒间隔轮询状态接口

### Requirement: 顺序上传多文件

系统 MUST 在接收方确认后按待发送列表顺序逐个上传文件，同一批文件共享同一个 `transferId`。

#### Scenario: 上传第一个文件
- **WHEN** 状态接口返回 `accepted`
- **THEN** 发送方系统 MUST 调用 `POST /api/transfer/upload` 上传待发送列表中的第一个文件

#### Scenario: 前一个文件完成后上传下一个
- **WHEN** 当前文件上传并保存成功
- **THEN** 发送方系统 MUST 继续上传待发送列表中的下一个文件

#### Scenario: 任一文件失败
- **WHEN** 任一文件上传或保存失败
- **THEN** 系统 MUST 将整个传输任务标记为失败

#### Scenario: 不并发上传
- **WHEN** 一个任务包含多个文件
- **THEN** 系统 MUST 不并发上传这些文件

### Requirement: 通过事件同步本机 UI 进度

系统 MUST 通过 Tauri event 将 Rust 层传输进度同步给本机 React UI。

#### Scenario: 发送进度更新
- **WHEN** Rust 层发送文件内容并计算出新的已发送字节数
- **THEN** 系统 MUST 发送包含 `transferId`、文件名、文件序号、已传字节、总字节和百分比的进度事件

#### Scenario: 接收进度更新
- **WHEN** Rust 层接收文件内容并写入保存目录
- **THEN** 系统 MUST 发送包含 `transferId`、文件名、已接收字节、总字节和百分比的进度事件

### Requirement: 使用流式文件传输

系统 MUST 在实现文件上传和保存时避免一次性将完整文件读入内存。

#### Scenario: 上传大文件
- **WHEN** 发送方上传大文件
- **THEN** Rust 层 MUST 以流式方式读取并发送文件内容

#### Scenario: 接收大文件
- **WHEN** 接收方保存大文件
- **THEN** Rust 层 MUST 以流式方式接收并写入文件

### Requirement: 处理传输异常

系统 MUST 在连接中断、请求失败、应用关闭或服务异常时将相关任务标记为失败。

#### Scenario: 上传过程中网络中断
- **WHEN** 文件上传过程中网络连接中断
- **THEN** 双方可感知的一侧 MUST 将任务标记为失败并展示基础错误提示

#### Scenario: 应用关闭导致中断
- **WHEN** 应用在传输过程中关闭
- **THEN** 本次传输 MUST 中断且 v0.1 不恢复该任务

#### Scenario: 发送中取消不可用
- **WHEN** 任务处于等待确认或传输中
- **THEN** 系统 MUST 不提供发送中取消能力
