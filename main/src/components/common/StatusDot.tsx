import clsx from "clsx";

export type StatusDotType = "primary" | "success" | "warning" | "danger" | "neutral";
export type StatusDotSize = "xs" | "sm" | "md" | "lg";

type StatusDotProps = {
  type?: StatusDotType;
  size?: StatusDotSize;
  className?: string;
  "aria-label"?: string;
};

const typeClassMap: Record<StatusDotType, string> = {
  primary: "bg-primary",
  success: "bg-success",
  warning: "bg-warning",
  danger: "bg-danger",
  neutral: "bg-neutral",
};

const sizeClassMap: Record<StatusDotSize, string> = {
  xs: "h-1.5 w-1.5",
  sm: "h-2 w-2",
  md: "h-2.5 w-2.5",
  lg: "h-3 w-3",
};

export function StatusDot({
  type = "neutral",
  size = "sm",
  className,
  "aria-label": ariaLabel,
}: StatusDotProps) {
  return (
    <span
      aria-label={ariaLabel}
      className={clsx(
        "inline-block shrink-0 rounded-full",
        typeClassMap[type],
        sizeClassMap[size],
        className,
      )}
      role={ariaLabel ? "img" : undefined}
    />
  );
}
