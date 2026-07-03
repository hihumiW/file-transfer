import type { ReactNode } from "react";

type InfoRowProps = {
  label: ReactNode;
  children: ReactNode;
};

export function InfoRow({ label, children }: InfoRowProps) {
  return (
    <div className="grid min-h-[38px] grid-cols-[88px_minmax(0,1fr)] items-center gap-2">
      <span className="text-[13px] text-fg-muted">{label}</span>
      <div className="grid min-w-0 grid-cols-[minmax(0,1fr)_auto] items-center gap-1.5">{children}</div>
    </div>
  );
}
