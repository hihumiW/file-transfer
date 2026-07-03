import { Button, ScrollShadow } from "@heroui/react";
import { FileText, Send, UploadCloud, X } from "lucide-react";
import { Panel } from "../common/Panel";
import { formatBytes } from "../../format";
import type { LocalFile } from "../../types";

type SendFilesPanelProps = {
  files: LocalFile[];
  selectedTotal: number;
  canSend: boolean;
  onChooseFiles: () => void;
  onRemoveFile: (path: string) => void;
  onSend: () => void;
};

export function SendFilesPanel({
  files,
  selectedTotal,
  canSend,
  onChooseFiles,
  onRemoveFile,
  onSend,
}: SendFilesPanelProps) {
  return (
    <Panel
      title="发送文件"
      className="min-h-0"
      bodyClassName="flex h-full flex-col p-5"
    >
      <Button
        variant="outline"
        fullWidth
        className={`${files.length === 0 ? "min-h-[260px] flex-1" : "h-[132px]"} w-full border-dashed border-border-dropzone bg-surface-muted text-fg-muted hover:border-primary hover:bg-primary-hover`}
        onPress={onChooseFiles}
      >
        <span className="grid justify-items-center gap-1">
          <span className="grid h-9 w-9 place-items-center rounded-full border border-primary text-primary">
            <UploadCloud className="h-5 w-5" />
          </span>
          <span className="text-[13px] font-semibold">
            拖拽文件到这里，或点击选择文件
          </span>
          <span className="text-[12px] text-fg-muted">
            支持多个文件同时发送
          </span>
        </span>
      </Button>

      {files.length > 0 && (
        <div className="mt-3 flex items-center justify-between gap-3">
          <h3 className="m-0 text-[13px] font-medium text-fg-muted">
            待发送（{files.length}）
          </h3>
          <span className="text-[13px] font-medium text-fg-default">
            {formatBytes(selectedTotal)}
          </span>
        </div>
      )}

      {files.length > 0 && (
        <ScrollShadow className="mt-2 min-h-[120px] flex-1">
          <div className="space-y-1.5">
            {files.map((file) => (
              <div
                key={file.path}
                className="grid min-h-9 grid-cols-[20px_minmax(0,1fr)_auto_28px] items-center gap-2 rounded-lg border border-border-subtle bg-surface px-2.5"
              >
                <FileText className="h-4 w-4 text-fg-muted" />
                <span className="text-ellipsis text-[13px] text-fg-default">
                  {file.name}
                </span>
                <span className="text-[12px] text-fg-muted">
                  {formatBytes(file.size)}
                </span>
                <Button
                  isIconOnly
                  size="sm"
                  variant="ghost"
                  onPress={() => onRemoveFile(file.path)}
                  aria-label="移除文件"
                >
                  <X className="h-4 w-4" />
                </Button>
              </div>
            ))}
          </div>
        </ScrollShadow>
      )}

      {files.length > 0 && (
        <div className="mt-3 flex justify-end">
          <Button variant="primary" isDisabled={!canSend} onPress={onSend}>
            <Send className="h-4 w-4" />
            发送 {files.length} 个文件
          </Button>
        </div>
      )}
    </Panel>
  );
}
