## ADDED Requirements

### Requirement: 展示本机设备信息

系统 MUST 在应用启动后展示当前设备名称、本机局域网访问地址和本地接收服务状态，并允许用户复制本机访问地址。

#### Scenario: 首次启动展示系统设备名
- **WHEN** 用户首次启动应用且系统设备名读取成功
- **THEN** 系统 MUST 展示系统设备名作为当前设备名称

#### Scenario: 用户自定义设备名优先
- **WHEN** 本地配置中存在用户自定义设备名
- **THEN** 系统 MUST 优先展示用户自定义设备名

#### Scenario: 设备名读取失败
- **WHEN** 系统设备名读取失败且用户未设置自定义设备名
- **THEN** 系统 MUST 展示默认兜底名称 `My Device`

#### Scenario: 保存有效自定义设备名
- **WHEN** 用户输入长度不超过 256 个字符且去除首尾空白后不为空的自定义设备名
- **THEN** 系统 MUST 保存该自定义设备名并优先展示

#### Scenario: 拒绝过长设备名
- **WHEN** 用户输入超过 256 个字符的自定义设备名
- **THEN** 系统 MUST 阻止保存并提示设备名称过长

#### Scenario: 拒绝空设备名
- **WHEN** 用户输入的自定义设备名去除首尾空白后为空
- **THEN** 系统 MUST 阻止保存并提示设备名称不能为空

#### Scenario: 复制本机访问地址
- **WHEN** 用户点击复制本机访问地址
- **THEN** 系统 MUST 将当前展示的 `http://IP:端口` 地址写入剪贴板

### Requirement: 生成并保存稳定设备标识

系统 MUST 在首次启动时生成 UUID v4 格式的 `deviceId` 并保存到本地配置，后续启动复用该标识。

#### Scenario: 首次启动生成 deviceId
- **WHEN** 本地配置不存在 `deviceId`
- **THEN** 系统 MUST 生成 UUID v4 并保存到本地配置

#### Scenario: 修改设备名不影响 deviceId
- **WHEN** 用户修改自定义设备名
- **THEN** 系统 MUST 保持已有 `deviceId` 不变

### Requirement: 推荐局域网 IPv4 地址

系统 MUST 枚举本机网络接口，推荐一个最可能用于局域网访问的 IPv4 地址，并展示候选地址列表。

#### Scenario: 过滤不可用地址
- **WHEN** 系统枚举网络接口
- **THEN** 系统 MUST 排除 loopback 地址、`169.254.x.x` link-local 地址、未启用接口和无有效 IPv4 的接口

#### Scenario: 优先展示私有地址
- **WHEN** 存在 `192.168.x.x`、`10.x.x.x` 或 `172.16.x.x` 至 `172.31.x.x` 私有 IPv4 地址
- **THEN** 系统 MUST 从私有 IPv4 地址中选择推荐访问地址

#### Scenario: 降低虚拟网卡优先级
- **WHEN** 候选地址来自 VPN、Docker、WSL、VirtualBox、VMware、Hyper-V 或 vEthernet 等虚拟网卡
- **THEN** 系统 MUST 降低该地址的推荐优先级

#### Scenario: 用户切换展示地址
- **WHEN** 用户从候选地址列表选择另一个 IPv4 地址
- **THEN** 系统 MUST 使用该地址更新 UI 展示和复制内容

### Requirement: 启动本地 HTTP 接收服务

系统 MUST 在应用启动时启动本地 HTTP 接收服务，服务实际监听 `0.0.0.0:实际端口`，UI 展示推荐 IPv4 地址加实际端口。

#### Scenario: 默认端口可用
- **WHEN** 端口 `7788` 可监听
- **THEN** 系统 MUST 使用 `0.0.0.0:7788` 启动本地服务

#### Scenario: 默认端口被占用
- **WHEN** 端口 `7788` 被占用
- **THEN** 系统 MUST 在 `7789` 至 `7888` 范围内按端口递增尝试监听

#### Scenario: 无可用端口
- **WHEN** `7788-7888` 范围内没有可用端口
- **THEN** 系统 MUST 将服务状态标记为异常并提示本地接收服务启动失败

#### Scenario: 切换展示 IP 不改变监听范围
- **WHEN** 用户切换候选展示 IP
- **THEN** 系统 MUST 继续监听 `0.0.0.0:实际端口`
