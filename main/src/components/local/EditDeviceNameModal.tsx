import { Button, Input } from "@heroui/react";
import { Check, Monitor, X } from "lucide-react";

type EditDeviceNameModalProps = {
  isOpen: boolean;
  value: string;
  onValueChange: (value: string) => void;
  onCancel: () => void;
  onSubmit: () => void;
};

export function EditDeviceNameModal({
  isOpen,
  value,
  onValueChange,
  onCancel,
  onSubmit,
}: EditDeviceNameModalProps) {
  if (!isOpen) {
    return null;
  }

  function handleKeyDown(event: React.KeyboardEvent<HTMLInputElement>) {
    if (event.key === "Enter") {
      onSubmit();
    }

    if (event.key === "Escape") {
      onCancel();
    }
  }

  return (
    <div
      className="fixed inset-0 z-50 grid place-items-center bg-black/45 px-6 backdrop-blur-sm"
      role="presentation"
      onMouseDown={onCancel}
    >
      <section
        aria-modal="true"
        className="w-full max-w-[460px] overflow-hidden rounded-2xl border border-border-subtle bg-surface shadow-xl"
        role="dialog"
        onMouseDown={(event) => event.stopPropagation()}
      >
        <header className="border-b border-border-subtle px-5 py-4">
          <div className="flex items-center gap-3">
            <span className="grid h-9 w-9 place-items-center rounded-full bg-primary-soft text-primary">
              <Monitor className="h-[18px] w-[18px]" />
            </span>
            <h3 className="m-0 text-[16px] font-semibold text-fg-default">
              修改设备名称
            </h3>
          </div>
        </header>
        <div className="px-5 py-5">
          <Input
            autoFocus
            value={value}
            maxLength={256}
            placeholder="请输入设备名称"
            onChange={(event) => onValueChange(event.currentTarget.value)}
            onKeyDown={handleKeyDown}
            fullWidth
            className="h-11 min-h-11 rounded-xl border border-border-control px-3 text-[14px]"
          />
          <p className="mt-4 text-[12px] leading-5 text-fg-muted">
            设备名称最多支持 256 个字符，会展示给同一局域网内的接收方。
          </p>
        </div>
        <footer className="flex justify-end gap-3 border-t border-border-subtle px-5 py-4">
          <Button variant="outline" onPress={onCancel}>
            <X className="h-4 w-4" />
            取消
          </Button>
          <Button variant="primary" onPress={onSubmit}>
            <Check className="h-4 w-4" />
            修改
          </Button>
        </footer>
      </section>
    </div>
  );
}
