import { invoke } from "@tauri-apps/api/core";
import type { AppSnapshot, LocalFile, TargetConnection } from "./types";

export function getAppSnapshot() {
  return invoke<AppSnapshot>("get_app_snapshot");
}

export function saveDeviceName(deviceName: string) {
  return invoke<AppSnapshot>("save_device_name", { deviceName });
}

export function selectDisplayIp(ip: string) {
  return invoke<AppSnapshot>("select_display_ip", { ip });
}

export function chooseSaveDir() {
  return invoke<AppSnapshot>("choose_save_dir");
}

export function openSaveDir() {
  return invoke<void>("open_save_dir");
}

export function chooseFiles() {
  return invoke<LocalFile[]>("choose_files");
}

export function describePaths(paths: string[]) {
  return invoke<LocalFile[]>("describe_paths", { paths });
}

export function testTargetConnection(raw: string) {
  return invoke<TargetConnection>("test_target_connection", { raw });
}

export function deleteRecentDevice(address: string) {
  return invoke<AppSnapshot>("delete_recent_device", { address });
}

export function sendFiles(targetAddress: string, files: LocalFile[]) {
  return invoke<AppSnapshot>("send_files", { targetAddress, files });
}

export function respondTransfer(accept: boolean, overwrite: boolean) {
  return invoke<AppSnapshot>("respond_transfer", { accept, overwrite });
}

export function clearCompletedTasks() {
  return invoke<AppSnapshot>("clear_completed_tasks");
}
