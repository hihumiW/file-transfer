export type NetworkKind = "wifi" | "ethernet" | "vpn" | "virtual" | "other";

export type DeviceInfo = {
  ok: boolean;
  deviceId: string;
  deviceName: string;
  version: string;
  protocolVersion: string;
  receiveEnabled: boolean;
};

export type NetworkAddress = {
  ip: string;
  label: string;
  kind: NetworkKind;
  recommended: boolean;
};

export type ServiceStatus = {
  running: boolean;
  port: number;
  message: string;
};

export type RecentDevice = {
  deviceId?: string;
  deviceName: string;
  address: string;
  ip: string;
  port: number;
  lastConnectedAt: number;
  lastSuccessAt?: number;
};

export type LocalFile = {
  path: string;
  name: string;
  size: number;
  isDir: boolean;
};

export type TransferFile = {
  name: string;
  size: number;
};

export type TransferStatus =
  | "pending"
  | "accepted"
  | "rejected"
  | "uploading"
  | "completed"
  | "failed";

export type TransferDirection = "send" | "receive";

export type TransferTask = {
  id: string;
  direction: TransferDirection;
  peerDeviceId?: string;
  peerDeviceName: string;
  peerAddress?: string;
  files: TransferFile[];
  totalBytes: number;
  transferredBytes: number;
  status: TransferStatus;
  message?: string;
  createdAt: number;
};

export type PendingTransfer = {
  transferId: string;
  senderDeviceId?: string;
  senderDeviceName: string;
  senderAddress: string;
  files: TransferFile[];
  totalBytes: number;
  status: TransferStatus;
  overwriteConfirmed: boolean;
  duplicateFiles: string[];
  createdAt: number;
};

export type AppSnapshot = {
  device: DeviceInfo;
  displayAddress: string;
  selectedIp?: string;
  networkAddresses: NetworkAddress[];
  service: ServiceStatus;
  saveDir: string;
  saveDirAvailable: boolean;
  recentDevices: RecentDevice[];
  tasks: TransferTask[];
  pendingTransfer?: PendingTransfer;
};

export type TargetConnection = {
  device: DeviceInfo;
  address: string;
  ip: string;
  port: number;
};

export type ProgressEvent = {
  transferId: string;
  fileName: string;
  fileIndex: number;
  fileTotal: number;
  transferredBytes: number;
  totalBytes: number;
  percent: number;
};

export type IncomingTransferEvent = {
  transfer: PendingTransfer;
};
