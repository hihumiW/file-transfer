import { Button, Input, ScrollShadow } from "@heroui/react";
import { Monitor, PlugZap, Server, Trash2 } from "lucide-react";
import { EmptyStateView } from "../common/EmptyStateView";
import { Panel } from "../common/Panel";
import { formatTime } from "../../format";
import type { AppSnapshot, TargetConnection } from "../../types";

type TargetConnectionPanelProps = {
  snapshot: AppSnapshot;
  targetInput: string;
  target?: TargetConnection;
  targetError?: string;
  targetTesting: boolean;
  activeSendLocked?: boolean;
  onTargetInputChange: (value: string) => void;
  onConnect: (address?: string) => void;
  onDeleteRecent: (address: string) => void;
};

export function TargetConnectionPanel({
  snapshot,
  targetInput,
  target,
  targetError,
  targetTesting,
  activeSendLocked,
  onTargetInputChange,
  onConnect,
  onDeleteRecent,
}: TargetConnectionPanelProps) {
  return (
    <Panel title="目标设备连接" bodyClassName="p-5">
      <div className="grid grid-cols-[minmax(0,1fr)_auto] gap-4">
        <Input
          value={targetInput}
          disabled={activeSendLocked}
          onChange={(event) => onTargetInputChange(event.currentTarget.value)}
          placeholder="输入对方 IP:端口，例如 192.168.1.8:7788"
          className="h-10 min-h-10 rounded-lg border border-border-control px-3 text-[13px] flex-1"
        />
        <Button variant="primary" isDisabled={!targetInput || targetTesting || activeSendLocked} onPress={() => onConnect()}>
          {!targetTesting && <PlugZap className="h-4 w-4" />}
          {targetTesting ? "连接中" : "连接"}
        </Button>
      </div>
      {target && <p className="mt-2 mb-0 text-[13px] text-success-strong">已连接：{target.device.deviceName} · {target.address}</p>}
      {targetError && <p className="mt-2 mb-0 text-[13px] text-danger-strong">{targetError}</p>}

      <h3 className="mt-4 mb-2 text-[13px] font-medium text-fg-muted">最近连接</h3>
      <ScrollShadow className="max-h-[140px]">
        <div className="space-y-2">
          {snapshot.recentDevices.length === 0 && <EmptyStateView text="暂无最近连接" icon={Server} className="min-h-[96px]" />}
          {snapshot.recentDevices.map((device) => (
            <div
              key={device.address}
              className="grid min-h-14 grid-cols-[28px_minmax(0,1fr)_auto_auto] items-center gap-2 rounded-lg border border-border-subtle bg-surface px-3 py-2"
            >
              <Monitor className="h-4 w-4 text-fg-muted" />
              <Button
                variant="ghost"
                isDisabled={activeSendLocked}
                className="h-auto min-w-0 justify-start p-0"
                onPress={() => onConnect(device.address)}
              >
                <span className="grid min-w-0 text-left">
                  <span className="text-ellipsis text-[14px] font-medium text-fg-default">{device.deviceName}</span>
                  <span className="text-ellipsis text-[12px] text-fg-muted">{device.address}</span>
                </span>
              </Button>
              <span className="whitespace-nowrap text-[12px] text-fg-muted">{formatTime(device.lastSuccessAt)}</span>
              <Button isIconOnly size="sm" variant="ghost" onPress={() => onDeleteRecent(device.address)} aria-label="删除最近连接">
                <Trash2 className="h-4 w-4" />
              </Button>
            </div>
          ))}
        </div>
      </ScrollShadow>
    </Panel>
  );
}
