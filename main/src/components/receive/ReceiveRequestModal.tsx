import { Button, Modal, ScrollShadow } from "@heroui/react";
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
    <Modal.Root isOpen>
      <Modal.Backdrop variant="blur" />
      <Modal.Container size="lg" placement="center">
        <Modal.Dialog className="rounded-2xl bg-white">
          <Modal.Header>
            <Modal.Heading className="text-[20px] font-semibold leading-7">接收文件请求</Modal.Heading>
          </Modal.Header>
          <Modal.Body>
            <div className="mt-1 grid grid-cols-[32px_minmax(0,1fr)_auto] items-start gap-3">
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
          </Modal.Body>
          <Modal.Footer className="grid grid-cols-2 gap-3 pt-6">
            <Button variant="outline" onPress={() => onRespond(false)}>
              <X className="h-4 w-4" />
              拒绝
            </Button>
            <Button variant="primary" onPress={() => onRespond(true, transfer.duplicateFiles.length > 0)}>
              <Check className="h-4 w-4" />
              接收
            </Button>
          </Modal.Footer>
        </Modal.Dialog>
      </Modal.Container>
    </Modal.Root>
  );
}
