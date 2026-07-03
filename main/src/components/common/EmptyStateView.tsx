import { EmptyState } from "@heroui/react";
import type { LucideIcon } from "lucide-react";
import { Inbox } from "lucide-react";

type EmptyStateViewProps = {
  text: string;
  icon?: LucideIcon;
  className?: string;
};

export function EmptyStateView({ text, icon: Icon = Inbox, className = "" }: EmptyStateViewProps) {
  return (
    <EmptyState className={`grid min-h-[120px] place-items-center py-6 text-fg-subtle ${className}`}>
      <div className="grid justify-items-center gap-2">
        <div className="grid h-12 w-12 place-items-center rounded-full bg-surface-soft text-fg-subtle">
          <Icon className="h-6 w-6" />
        </div>
        <span className="text-[13px] text-fg-subtle">{text}</span>
      </div>
    </EmptyState>
  );
}
