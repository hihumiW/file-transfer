import { create } from "zustand";
import type { AppSnapshot, LocalFile, PendingTransfer, ProgressEvent, TargetConnection } from "./types";
import { percent } from "./format";

type TaskFilter = "transferring" | "send" | "receive" | "completed";

type AppStore = {
  snapshot?: AppSnapshot;
  targetInput: string;
  target?: TargetConnection;
  targetError?: string;
  targetTesting: boolean;
  files: LocalFile[];
  filter: TaskFilter;
  busyMessage?: string;
  setSnapshot: (snapshot: AppSnapshot) => void;
  setTargetInput: (value: string) => void;
  setTarget: (target?: TargetConnection) => void;
  setTargetError: (message?: string) => void;
  setTargetTesting: (testing: boolean) => void;
  addFiles: (files: LocalFile[]) => void;
  removeFile: (path: string) => void;
  clearFiles: () => void;
  setFilter: (filter: TaskFilter) => void;
  applyProgress: (event: ProgressEvent) => void;
  setIncomingTransfer: (transfer: PendingTransfer) => void;
  setBusyMessage: (message?: string) => void;
};

export const useAppStore = create<AppStore>((set) => ({
  targetInput: "",
  targetTesting: false,
  files: [],
  filter: "transferring",
  setSnapshot: (snapshot) => set({ snapshot }),
  setTargetInput: (targetInput) => set({ targetInput, targetError: undefined }),
  setTarget: (target) => set({ target, targetInput: target?.address ?? "" }),
  setTargetError: (targetError) => set({ targetError }),
  setTargetTesting: (targetTesting) => set({ targetTesting }),
  addFiles: (incoming) =>
    set((state) => {
      const byPath = new Map(state.files.map((file) => [file.path, file]));
      incoming.filter((file) => !file.isDir).forEach((file) => byPath.set(file.path, file));
      return { files: [...byPath.values()] };
    }),
  removeFile: (path) => set((state) => ({ files: state.files.filter((file) => file.path !== path) })),
  clearFiles: () => set({ files: [] }),
  setFilter: (filter) => set({ filter }),
  applyProgress: (event) =>
    set((state) => {
      if (!state.snapshot) return state;
      return {
        snapshot: {
          ...state.snapshot,
          tasks: state.snapshot.tasks.map((task) => {
            if (task.id !== event.transferId) return task;

            const completedBeforeCurrentFile = task.files
              .slice(0, Math.max(event.fileIndex - 1, 0))
              .reduce((sum, file) => sum + file.size, 0);
            const nextTransferredBytes =
              event.totalBytes === task.totalBytes
                ? event.transferredBytes
                : completedBeforeCurrentFile + event.transferredBytes;

            return {
              ...task,
              transferredBytes: Math.min(task.totalBytes, nextTransferredBytes),
              status: event.status ?? (percent(event.transferredBytes, event.totalBytes) >= 100 ? task.status : "uploading"),
            };
          }),
        },
      };
    }),
  setIncomingTransfer: (transfer) =>
    set((state) => ({
      snapshot: state.snapshot ? { ...state.snapshot, pendingTransfer: transfer } : state.snapshot,
    })),
  setBusyMessage: (busyMessage) => set({ busyMessage }),
}));
