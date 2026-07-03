import { Button, InputGroup, ScrollShadow } from "@heroui/react";
import {
  Clipboard,
  Edit3,
  FolderOpen,
  HardDrive,
  Monitor,
  Network,
  Wifi,
} from "lucide-react";
import { InfoRow } from "../common/InfoRow";
import { Panel } from "../common/Panel";
import { StatusDot } from "../common/StatusDot";
import type { AppSnapshot } from "../../types";

type LocalStatusPanelProps = {
  snapshot: AppSnapshot;
  onStartEdit: () => void;
  onCopyAddress: () => void;
  onSelectIp: (ip: string) => void;
  onChooseSaveDir: () => void;
  onOpenSaveDir: () => void;
};

export function LocalStatusPanel({
  snapshot,
  onStartEdit,
  onCopyAddress,
  onSelectIp,
  onChooseSaveDir,
  onOpenSaveDir,
}: LocalStatusPanelProps) {
  return (
    <Panel className="h-full" bodyClassName="flex h-full flex-col p-0" title="">
      <section className="border-b border-border-subtle px-5 py-5">
        <PanelTitle title="本机状态" />
        <div className="mt-3 space-y-2">
          <InfoRow
            label={
              <p className="flex gap-x-2 items-center">
                <Monitor className="h-6 w-6" />
                <span>设备名称</span>
              </p>
            }
          >
            <strong className="text-ellipsis text-[13px] font-semibold text-fg-default text-right mr-1">
              {snapshot.device.deviceName}
            </strong>
            <Button
              isIconOnly
              size="sm"
              variant="ghost"
              onPress={onStartEdit}
              aria-label="编辑设备名称"
            >
              <Edit3 className="h-4 w-4" />
            </Button>
          </InfoRow>
          <InfoRow label="访问地址">
            <span className="flex min-w-0 items-center gap-2 text-[13px] font-medium text-primary-strong mr-1">
              <StatusDot type="primary" size="md" aria-label="访问地址" />
              <span className="text-ellipsis">{snapshot.displayAddress}</span>
            </span>
            <Button
              isIconOnly
              size="sm"
              variant="ghost"
              onPress={onCopyAddress}
              aria-label="复制访问地址"
            >
              <Clipboard className="h-4 w-4" />
            </Button>
          </InfoRow>
          <InfoRow label="服务状态">
            <span className="flex min-w-0 items-center gap-2 text-[13px] font-medium text-success-strong">
              <StatusDot type={snapshot.service.running ? "success" : "danger"} size="md" aria-label="服务状态" />
              <span className="text-ellipsis">{snapshot.service.message}</span>
            </span>
            <span />
          </InfoRow>
        </div>
      </section>

      <section className="border-b border-border-subtle px-5 py-5">
        <PanelTitle icon={<Wifi className="h-5 w-5 mr-2" />} title="网络" />
        <ScrollShadow className="mt-4 max-h-[230px]">
          <div className="space-y-3 p-1">
            {snapshot.networkAddresses.map((item) => {
              const selected = snapshot.selectedIp === item.ip;
              return (
                <Button
                  key={`${item.label}-${item.ip}`}
                  fullWidth
                  variant={selected ? "primary" : "outline"}
                  className={`h-auto justify-start px-4 py-3 outline-offset-[-2px] focus-visible:outline-2 focus-visible:outline-focus ${selected ? "bg-primary-soft text-primary" : "bg-surface text-fg-default"}`}
                  onPress={() => onSelectIp(item.ip)}
                >
                  <Network className="h-5 w-5 shrink-0 mr-2" />
                  <span className="grid min-w-0 text-left">
                    <span className="text-ellipsis text-[13px] font-semibold">
                      {item.ip}
                    </span>
                    <span className="text-ellipsis text-[12px] text-fg-muted">
                      {item.label}
                      {item.recommended ? "（推荐）" : ""}
                    </span>
                  </span>
                </Button>
              );
            })}
          </div>
        </ScrollShadow>
      </section>

      <section className="px-5 py-5">
        <PanelTitle icon={<HardDrive className="h-5 w-5 mr-2" />} title="保存目录" />
        <InputGroup
          fullWidth
          variant="secondary"
          className="mt-5 h-10 rounded-lg border border-border-subtle bg-surface-muted"
        >
          <InputGroup.Input
            readOnly
            value={snapshot.saveDir}
            className="text-ellipsis text-[13px] text-fg-default"
          />
          <InputGroup.Suffix>
            <Button
              isIconOnly
              size="sm"
              variant="ghost"
              aria-label="选择保存目录"
              onPress={onChooseSaveDir}
            >
              <FolderOpen className="h-4.5 w-4.5" />
            </Button>
          </InputGroup.Suffix>
        </InputGroup>
        <Button
          className="mt-3 justify-center text-primary hover:bg-primary-soft"
          size="sm"
          variant="ghost"
          onPress={onOpenSaveDir}
        >
          <FolderOpen className="h-4 w-4" />
          打开保存目录
        </Button>
      </section>
    </Panel>
  );
}

function PanelTitle({
  icon,
  title,
}: {
  icon?: React.ReactNode;
  title: string;
}) {
  return (
    <h2 className="m-0 flex items-center gap-2 text-[15px] font-semibold leading-[22px] text-fg-default">
      {icon ? <span className="text-fg-muted">{icon}</span> : null}
      {title}
    </h2>
  );
}
