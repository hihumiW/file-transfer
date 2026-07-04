import { Button, ScrollShadow } from "@heroui/react";
import { FileText, Monitor, X, Check } from "lucide-react";
import { formatBytes } from "../../format";
import type { PendingTransfer } from "../../types";

type ReceiveRequestModalProps = {
  transfer?: PendingTransfer;
  onRespond: (accept: boolean, overwrite?: boolean) => void;
};

export function ReceiveRequestModal({ transfer, onRespond }: ReceiveRequestModalProps) {
  if (transfer?.status !== "pending") return null;

  return (
    // 使用手写 fixed 弹窗，避免复合 Modal 的 backdrop 与内容层级在桌面壳中错位。
    <div className="fixed inset-0 z-50 grid place-items-center bg-black/45 px-6 backdrop-blur-sm" role="presentation">
      <section
        aria-modal="true"
        className="w-full max-w-[560px] overflow-hidden rounded-2xl border border-border-subtle bg-surface shadow-xl"
        role="dialog"
      >
        {/* 头部只承载标题，保持接收确认弹窗的视觉重心清晰。 */}
        <header className="border-b border-border-subtle px-5 py-4">
          <h3 className="m-0 text-[18px] font-semibold leading-7 text-fg-default">接收文件请求</h3>
        </header>

        {/* 主体展示发送方、文件数量、文件列表和重名风险。 */}
        <div className="px-5 py-5">
          <div className="grid grid-cols-[32px_minmax(0,1fr)_auto] items-start gap-3">
            <Monitor className="mt-1 h-6 w-6 text-fg-muted" />
            <div className="min-w-0">
              <div className="flex min-w-0 items-center gap-2">
                <strong className="text-ellipsis text-[15px] font-semibold text-fg-default">{transfer.senderDeviceName}</strong>
                <span className="text-ellipsis text-[13px] text-fg-muted">{transfer.senderAddress}</span>
              </div>
              <p className="mt-1 mb-0 text-[13px] text-fg-muted">
                对方希望发送 {transfer.files.length} 个文件（{formatBytes(transfer.totalBytes)}）
              </p>
            </div>
            <span className="whitespace-nowrap text-[13px] text-fg-muted">现在</span>
          </div>

          <ScrollShadow className="mt-5 max-h-[240px] border-y border-border-subtle">
            {transfer.files.map((file) => (
              <div key={file.name} className="grid min-h-16 grid-cols-[28px_minmax(0,1fr)_auto] items-center gap-3 border-b border-border-subtle last:border-b-0">
                <FileText className="h-5 w-5 text-fg-muted" />
                <span className="text-ellipsis text-[14px] font-medium text-fg-default">{file.name}</span>
                <b className="text-[13px] font-medium text-fg-muted">{formatBytes(file.size)}</b>
              </div>
            ))}
          </ScrollShadow>

          {transfer.duplicateFiles.length > 0 && (
            <p className="mt-3 mb-0 rounded-lg bg-warning-soft px-3 py-2 text-[12px] text-warning-strong">
              存在同名文件：{transfer.duplicateFiles.join("、")}。接收将覆盖这些文件。
            </p>
          )}
        </div>

        {/* 底部只提供明确选择，不允许点击遮罩误关闭待确认请求。 */}
        <footer className="flex justify-end items-center gap-3 border-t border-border-subtle px-5 py-4">
          <Button variant="outline" onPress={() => onRespond(false)}>
            <X className="h-4 w-4" />
            拒绝
          </Button>
          <Button variant="primary" onPress={() => onRespond(true, transfer.duplicateFiles.length > 0)}>
            <Check className="h-4 w-4" />
            接收
          </Button>
        </footer>
      </section>
    </div>
  );
}
