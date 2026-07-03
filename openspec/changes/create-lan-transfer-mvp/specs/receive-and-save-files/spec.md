## ADDED Requirements

### Requirement: 展示接收确认请求

系统 MUST 在接收方收到传输请求后向本机用户展示确认信息，并允许用户接收或拒绝整个任务。

#### Scenario: 展示请求信息
- **WHEN** 接收方收到有效传输请求
- **THEN** 系统 MUST 展示发送方设备名、发送方地址、文件名、文件数量和总大小

#### Scenario: 用户接收
- **WHEN** 接收方用户点击接收
- **THEN** 系统 MUST 将任务状态更新为 `accepted` 并允许发送方上传文件

#### Scenario: 用户拒绝
- **WHEN** 接收方用户点击拒绝
- **THEN** 系统 MUST 将任务状态更新为 `rejected` 并使发送方任务显示已拒绝

#### Scenario: 不支持部分接收
- **WHEN** 一个任务包含多个文件
- **THEN** 系统 MUST 仅支持接收全部文件或拒绝整个任务

### Requirement: 限制接收方单任务处理

系统 MUST 在 v0.1 中限制接收方同一时间只处理一个待确认或传输中的任务。

#### Scenario: 空闲时接收请求
- **WHEN** 接收方没有待确认或传输中的任务
- **THEN** 系统 MUST 接收新的传输请求并进入待确认状态

#### Scenario: 待确认时收到新请求
- **WHEN** 接收方已有待确认任务
- **THEN** 系统 MUST 拒绝新的传输请求并返回忙碌状态

#### Scenario: 传输中收到新请求
- **WHEN** 接收方已有传输中任务
- **THEN** 系统 MUST 拒绝新的传输请求并返回忙碌状态

### Requirement: 管理保存目录

系统 MUST 将接收文件保存到当前保存目录，默认目录为系统下载目录下的 `LanTransfer` 文件夹，并允许用户修改默认保存目录。

#### Scenario: 使用默认保存目录
- **WHEN** 用户未设置自定义保存目录
- **THEN** 系统 MUST 使用系统下载目录下的 `LanTransfer` 文件夹

#### Scenario: 使用自定义保存目录
- **WHEN** 用户设置了自定义保存目录
- **THEN** 系统 MUST 将后续接收文件保存到该目录

#### Scenario: 修改保存目录
- **WHEN** 用户通过系统目录选择器选择新的保存目录
- **THEN** 系统 MUST 保存该目录到本地配置

#### Scenario: 打开保存目录
- **WHEN** 用户点击打开保存目录
- **THEN** 系统 MUST 调用系统能力打开当前保存目录

### Requirement: 检查保存目录可用性

系统 MUST 在应用启动或首次接收文件时检查保存目录可用性，并将 `receiveEnabled` 与保存目录可用性保持一致。

#### Scenario: 保存目录不存在
- **WHEN** 当前保存目录不存在
- **THEN** 系统 MUST 尝试自动创建该目录

#### Scenario: 保存目录创建失败
- **WHEN** 保存目录不存在且自动创建失败
- **THEN** 系统 MUST 提示用户重新选择保存目录并设置 `receiveEnabled=false`

#### Scenario: 保存目录无写入权限
- **WHEN** 当前保存目录不可写
- **THEN** 系统 MUST 提示保存目录不可用并设置 `receiveEnabled=false`

#### Scenario: 保存目录可用
- **WHEN** 当前保存目录存在且可写
- **THEN** 系统 MUST 设置 `receiveEnabled=true`

### Requirement: 处理重名文件

系统 MUST 在接收方确认前检查保存目录中的同名文件，并按任务级别处理重名冲突。

#### Scenario: 单文件重名
- **WHEN** 保存目录中存在与待接收文件同名的文件
- **THEN** 系统 MUST 展示重名确认并允许用户覆盖或取消本次传输

#### Scenario: 多文件存在重名
- **WHEN** 多文件任务中存在一个或多个重名文件
- **THEN** 系统 MUST 展示重名文件列表并允许覆盖全部重名文件或取消本次传输

#### Scenario: 用户选择覆盖
- **WHEN** 用户选择覆盖重名文件
- **THEN** 系统 MUST 继续接收任务并覆盖全部重名文件

#### Scenario: 用户选择取消
- **WHEN** 用户在重名确认中选择取消
- **THEN** 系统 MUST 取消本次传输任务

#### Scenario: 不支持自动重命名
- **WHEN** 接收文件与保存目录中文件重名
- **THEN** 系统 MUST 不自动生成新文件名

### Requirement: 处理保存失败

系统 MUST 在文件保存失败时展示失败原因，并将任务标记为失败。

#### Scenario: 磁盘空间不足
- **WHEN** 文件写入过程中磁盘空间不足
- **THEN** 系统 MUST 将任务标记为失败并提示保存失败原因

#### Scenario: 写入中断
- **WHEN** 文件写入过程中应用关闭、网络中断或服务异常
- **THEN** 系统 MUST 将任务标记为失败
