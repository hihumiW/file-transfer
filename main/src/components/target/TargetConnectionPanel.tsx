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
              className="grid min-h-14 grid-cols-[minmax(0,1fr)_auto] items-stretch overflow-hidden rounded-lg border border-border-subtle bg-surface transition-colors focus-within:border-primary/50 hover:border-primary/40 hover:bg-primary-hover"
            >
              {/* 将最近设备主体做成整行按钮，扩大可点击热区并统一 hover 反馈。 */}
              <button
                type="button"
                disabled={activeSendLocked}
                className="grid min-h-14 min-w-0 appearance-none grid-cols-[28px_minmax(0,1fr)_auto] items-center gap-2 border-0 bg-transparent px-3 py-2 text-left outline-none transition-colors disabled:cursor-not-allowed disabled:opacity-60"
                onClick={() => onConnect(device.address)}
              >
                <Monitor className="h-4 w-4 text-fg-muted" />
                <span className="grid min-w-0">
                  <span className="text-ellipsis text-[14px] font-medium text-fg-default">{device.deviceName}</span>
                  <span className="text-ellipsis text-[12px] text-fg-muted">{device.address}</span>
                </span>
                <span className="whitespace-nowrap text-[12px] text-fg-muted">{formatTime(device.lastSuccessAt)}</span>
              </button>
              {/* 删除是独立操作，避免按钮嵌套，同时维持整行主体的点击体验。 */}
              <Button isIconOnly size="sm" variant="ghost" className="m-2 self-center" onPress={() => onDeleteRecent(device.address)} aria-label="删除最近连接">
                <Trash2 className="h-4 w-4" />
              </Button>
            </div>
          ))}
        </div>
      </ScrollShadow>
    </Panel>
  );
}
