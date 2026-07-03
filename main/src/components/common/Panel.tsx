import type { ReactNode } from "react";

type PanelProps = {
  title?: string;
  actions?: ReactNode;
  className?: string;
  bodyClassName?: string;
  children: ReactNode;
};

export function Panel({ title, actions, className = "", bodyClassName = "", children }: PanelProps) {
  return (
    <section
      className={`min-w-0 overflow-hidden rounded-2xl border border-border-subtle bg-surface shadow-[0_1px_2px_rgba(15,23,42,0.04)] ${className}`}
    >
      <div className={`${bodyClassName}`}>
        {(title || actions) && (
          <div className="mb-3.5 flex min-h-7 items-center justify-between gap-3">
            {title && <h2 className="m-0 text-[15px] font-semibold leading-[22px] text-fg-default">{title}</h2>}
            {actions}
          </div>
        )}
        {children}
      </div>
    </section>
  );
}
