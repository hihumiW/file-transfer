## ADDED Requirements

### Requirement: 规范化目标地址输入

系统 MUST 支持用户输入 IPv4、IPv4 加端口或 HTTP URL，并将其规范化为 v0.1 支持的 HTTP 地址。

#### Scenario: 输入纯 IPv4
- **WHEN** 用户输入 `192.168.1.23`
- **THEN** 系统 MUST 规范化为 `http://192.168.1.23:7788`

#### Scenario: 输入 IPv4 加端口
- **WHEN** 用户输入 `192.168.1.23:7790`
- **THEN** 系统 MUST 规范化为 `http://192.168.1.23:7790`

#### Scenario: 输入缺少端口的 HTTP URL
- **WHEN** 用户输入 `http://192.168.1.23`
- **THEN** 系统 MUST 规范化为 `http://192.168.1.23:7788`

#### Scenario: 输入完整 HTTP URL
- **WHEN** 用户输入 `http://192.168.1.23:7790`
- **THEN** 系统 MUST 保留该端口并使用 `http://192.168.1.23:7790`

#### Scenario: 输入不支持的地址
- **WHEN** 用户输入非 IPv4 地址、HTTPS 地址或格式错误的地址
- **THEN** 系统 MUST 阻止连接测试并提示地址格式不正确

### Requirement: 执行目标设备连接测试

系统 MUST 在发送文件前要求用户对目标地址完成一次成功的连接测试。

#### Scenario: 连接测试成功
- **WHEN** 系统请求目标地址的 `GET /api/device` 并收到兼容响应且 `receiveEnabled` 为 `true`
- **THEN** 系统 MUST 展示目标设备名称、规范化地址和已连接状态

#### Scenario: 请求超时
- **WHEN** `GET /api/device` 请求超时
- **THEN** 系统 MUST 提示连接超时并说明确认对方设备已启动且处于同一局域网

#### Scenario: 连接被拒绝
- **WHEN** 目标地址拒绝连接或端口不可访问
- **THEN** 系统 MUST 提示无法连接到该地址并建议检查端口或防火墙

#### Scenario: 返回内容不匹配
- **WHEN** `GET /api/device` 返回内容不符合本应用协议
- **THEN** 系统 MUST 提示目标地址不是有效的局域网传输工具

#### Scenario: 协议版本不兼容
- **WHEN** 目标设备 `protocolVersion` 与当前应用不兼容
- **THEN** 系统 MUST 提示对方应用版本暂不兼容

#### Scenario: 对方不可接收
- **WHEN** 目标设备返回 `receiveEnabled=false`
- **THEN** 系统 MUST 提示对方设备当前不可接收文件

### Requirement: 提供设备信息接口

系统 MUST 通过 `GET /api/device` 返回当前设备的基本信息、应用版本、协议版本和接收可用状态。

#### Scenario: 设备信息响应
- **WHEN** 其他设备请求 `GET /api/device`
- **THEN** 系统 MUST 返回 `ok`、`deviceId`、`deviceName`、`version`、`protocolVersion` 和 `receiveEnabled`

#### Scenario: 保存目录不可用
- **WHEN** 当前保存目录不可用
- **THEN** 系统 MUST 在设备信息中返回 `receiveEnabled=false`

### Requirement: 管理最近连接设备

系统 MUST 在连接测试成功后保存或更新最近连接设备，并允许用户快速填入和删除记录。

#### Scenario: 保存成功连接
- **WHEN** 连接测试成功
- **THEN** 系统 MUST 保存设备名称、规范化地址、IP、端口、连接时间和可用的 `deviceId`

#### Scenario: 更新相同设备记录
- **WHEN** 最近连接列表已存在相同 `deviceId` 的记录
- **THEN** 系统 MUST 更新原记录而不是新增重复记录

#### Scenario: 按地址更新无 deviceId 记录
- **WHEN** 记录缺少 `deviceId` 但存在相同地址
- **THEN** 系统 MUST 更新原地址记录

#### Scenario: 最近连接数量限制
- **WHEN** 最近连接记录超过 10 条
- **THEN** 系统 MUST 仅保留最近成功连接时间倒序的前 10 条

#### Scenario: 点击最近连接
- **WHEN** 用户点击最近连接设备
- **THEN** 系统 MUST 填入该设备地址并要求重新执行连接测试

### Requirement: 限制当前发送目标切换

系统 MUST 将目标连接区视为当前发送目标，并在存在等待确认或传输中的发送任务时禁止切换。

#### Scenario: 无进行中发送任务时切换
- **WHEN** 当前没有等待确认或传输中的发送任务
- **THEN** 系统 MUST 允许用户测试并切换当前发送目标

#### Scenario: 存在进行中发送任务时锁定
- **WHEN** 当前存在等待确认或传输中的发送任务
- **THEN** 系统 MUST 锁定目标连接区并禁止切换当前发送目标

#### Scenario: 切换目标不清空任务
- **WHEN** 用户成功切换当前发送目标
- **THEN** 系统 MUST 保留运行期传输任务列表中的既有任务
