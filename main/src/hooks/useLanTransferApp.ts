import { useEffect, useMemo, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  chooseFiles,
  chooseSaveDir,
  clearCompletedTasks,
  deleteRecentDevice,
  describePaths,
  getAppSnapshot,
  openSaveDir,
  respondTransfer,
  saveDeviceName,
  selectDisplayIp,
  sendFiles,
  testTargetConnection,
} from "../api";
import { useAppStore } from "../store";
import type { IncomingTransferEvent, ProgressEvent } from "../types";

export function useLanTransferApp() {
  const store = useAppStore();
  const [editingName, setEditingName] = useState(false);
  const [deviceNameDraft, setDeviceNameDraft] = useState("");

  useEffect(() => {
    refresh();

    const progressUnlisten = listen<ProgressEvent>("transfer-progress", (event) => {
      store.applyProgress(event.payload);
      refresh();
    });
    const incomingUnlisten = listen<IncomingTransferEvent>("incoming-transfer", (event) => {
      store.setIncomingTransfer(event.payload.transfer);
      refresh();
    });
    const dragDropUnlisten = getCurrentWindow().onDragDropEvent(async (event) => {
      if (event.payload.type === "drop") {
        const dropped = await describePaths(event.payload.paths);
        console.log(dropped, 'droped');
        store.addFiles(dropped);
      }
    });

    return () => {
      progressUnlisten.then((unlisten) => unlisten());
      incomingUnlisten.then((unlisten) => unlisten());
      dragDropUnlisten.then((unlisten) => unlisten());
    };
  }, []);

  const selectedTotal = useMemo(() => store.files.reduce((sum, file) => sum + file.size, 0), [store.files]);
  const activeSendLocked = store.snapshot?.tasks.some(
    (task) => task.direction === "send" && ["pending", "accepted", "uploading"].includes(task.status),
  );
  const filteredTasks = useMemo(() => {
    const tasks = store.snapshot?.tasks ?? [];
    if (store.filter === "completed") {
      return tasks.filter((task) => ["completed", "failed", "rejected"].includes(task.status));
    }
    if (store.filter === "send" || store.filter === "receive") {
      return tasks.filter((task) => task.direction === store.filter);
    }
    return tasks;
  }, [store.filter, store.snapshot?.tasks]);

  async function refresh() {
    store.setSnapshot(await getAppSnapshot());
  }

  async function handleSaveDeviceName() {
    try {
      const next = await saveDeviceName(deviceNameDraft);
      store.setSnapshot(next);
      setEditingName(false);
    } catch (err) {
      store.setBusyMessage(String(err));
    }
  }

  async function handleChooseFiles() {
    const selected = await chooseFiles();
    store.addFiles(selected);
  }

  async function handleTestConnection(address = store.targetInput) {
    store.setTargetTesting(true);
    store.setTargetError(undefined);
    try {
      const connected = await testTargetConnection(address);
      store.setTarget(connected);
      store.setSnapshot(await getAppSnapshot());
    } catch (err) {
      store.setTarget(undefined);
      store.setTargetError(String(err));
    } finally {
      store.setTargetTesting(false);
    }
  }

  async function handleSend() {
    if (!store.target || store.files.length === 0) return;
    store.setBusyMessage("正在等待对方确认...");
    try {
      const next = await sendFiles(store.target.address, store.files);
      console.log(next, 'next');
      store.setSnapshot(next);
      store.clearFiles();
    } catch (err) {
      console.log('errrrrrrrr-');
      store.setBusyMessage(String(err));
      store.setSnapshot(await getAppSnapshot());
    }
  }

  async function handleRespond(accept: boolean, overwrite = false) {
    try {
      store.setSnapshot(await respondTransfer(accept, overwrite));
    } catch (err) {
      store.setBusyMessage(String(err));
    }
  }

  async function handleSelectIp(ip: string) {
    store.setSnapshot(await selectDisplayIp(ip));
  }

  async function handleChooseSaveDir() {
    store.setSnapshot(await chooseSaveDir());
  }

  async function handleDeleteRecent(address: string) {
    store.setSnapshot(await deleteRecentDevice(address));
  }

  async function handleClearCompleted() {
    store.setSnapshot(await clearCompletedTasks());
  }

  return {
    ...store,
    editingName,
    deviceNameDraft,
    selectedTotal,
    activeSendLocked,
    filteredTasks,
    setDeviceNameDraft,
    startEditingName: () => {
      setDeviceNameDraft(store.snapshot?.device.deviceName ?? "");
      setEditingName(true);
    },
    cancelEditingName: () => {
      setDeviceNameDraft(store.snapshot?.device.deviceName ?? "");
      setEditingName(false);
    },
    handleSaveDeviceName,
    handleChooseFiles,
    handleTestConnection,
    handleSend,
    handleRespond,
    handleSelectIp,
    handleChooseSaveDir,
    handleOpenSaveDir: openSaveDir,
    handleDeleteRecent,
    handleClearCompleted,
    handleCopyAddress: () => navigator.clipboard.writeText(store.snapshot?.displayAddress ?? ""),
  };
}
