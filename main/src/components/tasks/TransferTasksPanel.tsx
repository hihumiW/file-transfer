import { Button, Chip, ProgressBar, ScrollShadow, Tabs } from "@heroui/react";
import {
  CheckCircle2,
  Clock3,
  FileText,
  Inbox,
  ListChecks,
  Send,
  Trash2,
  XCircle,
} from "lucide-react";
import { EmptyStateView } from "../common/EmptyStateView";
import { Panel } from "../common/Panel";
import { formatBytes, percent } from "../../format";
import type { TransferStatus, TransferTask } from "../../types";

type TaskFilter = "all" | "send" | "receive" | "completed";

const statusText: Record<TransferStatus, string> = {
  pending: "等待确认",
  accepted: "已接收",
  rejected: "已拒绝",
  uploading: "传输中",
  completed: "完成",
  failed: "失败",
};

type TransferTasksPanelProps = {
  tasks: TransferTask[];
  filter: TaskFilter;
  onFilterChange: (filter: TaskFilter) => void;
  onClearCompleted: () => void;
};

export function TransferTasksPanel({
  tasks,
  filter,
  onFilterChange,
  onClearCompleted,
}: TransferTasksPanelProps) {
  return (
    <Panel
      title="传输任务"
      className="h-full"
      bodyClassName="flex h-full flex-col p-5"
      actions={
        <Button
          isIconOnly
          size="sm"
          variant="ghost"
          onPress={onClearCompleted}
          aria-label="清空已完成"
        >
          <Trash2 className="h-4 w-4" />
        </Button>
      }
    >
      <div className="pb-3">
        <Tabs
          selectedKey={filter}
          onSelectionChange={(key) => onFilterChange(key as TaskFilter)}
        >
          <Tabs.ListContainer>
            <Tabs.List aria-label="传输任务筛选">
              <Tabs.Tab id="all">
                全部
                <Tabs.Indicator />
              </Tabs.Tab>
              <Tabs.Tab id="send">
                发送
                <Tabs.Indicator />
              </Tabs.Tab>
              <Tabs.Tab id="receive">
                接收
                <Tabs.Indicator />
              </Tabs.Tab>
              <Tabs.Tab id="completed">
                已完成
                <Tabs.Indicator />
              </Tabs.Tab>
            </Tabs.List>
          </Tabs.ListContainer>
        </Tabs>
      </div>

      <ScrollShadow className="min-h-0 flex-1 px-5 pb-5">
        <div className="space-y-2">
          {tasks.length === 0 && (
            <EmptyStateView
              text="暂无任务"
              icon={ListChecks}
              className="min-h-[180px]"
            />
          )}
          {tasks.map((task) => {
            const taskPercent = percent(task.transferredBytes, task.totalBytes);
            return (
              <article
                key={task.id}
                className="rounded-lg border border-border-subtle bg-surface p-3 hover:bg-surface-muted"
              >
                <div className="grid grid-cols-[24px_minmax(0,1fr)_auto] items-start gap-2.5">
                  <TaskDirectionIcon task={task} />
                  <div className="min-w-0">
                    <div className="text-ellipsis text-[13px] font-semibold text-fg-default">
                      {task.files[0]?.name ?? "未知文件"}
                    </div>
                    <div className="text-ellipsis text-[12px] text-fg-muted">
                      {task.direction === "send"
                        ? `发送给 ${task.peerDeviceName}`
                        : `从 ${task.peerDeviceName} 接收`}{" "}
                      · {formatBytes(task.totalBytes)}
                    </div>
                  </div>
                  <StatusChip status={task.status} />
                </div>
                {task.status === "uploading" && (
                  <ProgressBar
                    aria-label="传输进度"
                    value={taskPercent}
                    size="sm"
                    className="mt-2"
                    color="accent"
                  />
                )}
                {task.message && (
                  <p className="mt-2 mb-0 text-[12px] text-danger-strong">
                    {task.message}
                  </p>
                )}
              </article>
            );
          })}
        </div>
      </ScrollShadow>
    </Panel>
  );
}

function TaskDirectionIcon({ task }: { task: TransferTask }) {
  const iconClass = "h-4 w-4";
  if (task.direction === "send")
    return <Send className={`${iconClass} text-primary`} />;
  return <Inbox className={`${iconClass} text-success-strong`} />;
}

function StatusChip({ status }: { status: TransferStatus }) {
  const icon =
    status === "completed" ? (
      <CheckCircle2 className="h-3 w-3" />
    ) : status === "failed" || status === "rejected" ? (
      <XCircle className="h-3 w-3" />
    ) : status === "pending" || status === "accepted" ? (
      <Clock3 className="h-3 w-3" />
    ) : (
      <FileText className="h-3 w-3" />
    );
  const color =
    status === "completed"
      ? "success"
      : status === "failed"
        ? "danger"
        : status === "rejected"
          ? "default"
          : status === "uploading"
            ? "accent"
            : "warning";
  return (
    <Chip
      size="sm"
      variant="soft"
      color={color}
      className="h-6 px-1 text-[12px] font-semibold"
    >
      <span className="mr-1 inline-flex">{icon}</span>
      {statusText[status]}
    </Chip>
  );
}
