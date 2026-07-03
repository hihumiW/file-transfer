## ADDED Requirements

### Requirement: 展示运行期传输任务列表

系统 MUST 展示当前应用运行期间的发送任务和接收任务，应用关闭后任务列表清空。

#### Scenario: 新增发送任务
- **WHEN** 用户发起文件发送
- **THEN** 系统 MUST 在传输任务列表中新增一条发送任务

#### Scenario: 新增接收任务
- **WHEN** 接收方收到有效传输请求
- **THEN** 系统 MUST 在传输任务列表中新增一条接收任务

#### Scenario: 应用重启
- **WHEN** 应用关闭后重新启动
- **THEN** 系统 MUST 不展示上一次运行的任务历史

#### Scenario: 切换目标不清空任务
- **WHEN** 用户切换当前发送目标
- **THEN** 系统 MUST 保留传输任务列表中的既有任务

### Requirement: 记录任务方向和对方设备

系统 MUST 为每条传输任务记录方向、对方设备名称、对方地址和文件信息。

#### Scenario: 发送任务方向展示
- **WHEN** 任务方向为 `send`
- **THEN** 系统 MUST 使用“发送给 {peerDeviceName}”表达任务方向

#### Scenario: 接收任务方向展示
- **WHEN** 任务方向为 `receive`
- **THEN** 系统 MUST 使用“从 {peerDeviceName} 接收”表达任务方向

#### Scenario: 多文件任务信息
- **WHEN** 一个任务包含多个文件
- **THEN** 系统 MUST 记录文件列表、总字节数和已传输字节数

### Requirement: 管理任务状态

系统 MUST 以任务为单位展示等待确认、已拒绝、传输中、成功和失败状态。

#### Scenario: 发送请求后等待确认
- **WHEN** 发送方提交传输请求且接收方尚未确认
- **THEN** 系统 MUST 将发送任务状态展示为等待确认

#### Scenario: 接收方拒绝
- **WHEN** 接收方拒绝传输任务
- **THEN** 系统 MUST 将对应发送任务状态展示为已拒绝

#### Scenario: 文件上传中
- **WHEN** 文件内容正在上传或接收
- **THEN** 系统 MUST 将任务状态展示为传输中

#### Scenario: 全部文件成功
- **WHEN** 一个任务中的全部文件上传并保存成功
- **THEN** 系统 MUST 将任务状态展示为成功

#### Scenario: 任一文件失败
- **WHEN** 一个任务中的任一文件上传或保存失败
- **THEN** 系统 MUST 将任务状态展示为失败

### Requirement: 展示任务进度

系统 MUST 基于已传输字节数和总字节数展示任务进度。

#### Scenario: 进度更新
- **WHEN** 系统收到传输进度事件
- **THEN** 系统 MUST 更新对应任务的已传输字节数和进度展示

#### Scenario: 多文件整体进度
- **WHEN** 任务包含多个文件
- **THEN** 系统 MUST 以任务总字节数为基准展示整体进度

### Requirement: 提供完成后基础操作

系统 MUST 在接收完成后允许用户打开保存目录，并允许用户清空已完成任务的展示。

#### Scenario: 接收完成打开目录
- **WHEN** 接收任务成功完成
- **THEN** 系统 MUST 提供打开保存目录操作

#### Scenario: 清空已完成任务
- **WHEN** 用户点击清空已完成
- **THEN** 系统 MUST 从任务列表中移除成功、失败或已拒绝的任务
