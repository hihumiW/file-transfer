import { Spinner } from "@heroui/react";
import { AppLayout } from "./components/layout/AppLayout";
import { EditDeviceNameModal } from "./components/local/EditDeviceNameModal";
import { LocalStatusPanel } from "./components/local/LocalStatusPanel";
import { TargetConnectionPanel } from "./components/target/TargetConnectionPanel";
import { SendFilesPanel } from "./components/send/SendFilesPanel";
import { TransferTasksPanel } from "./components/tasks/TransferTasksPanel";
import { ReceiveRequestModal } from "./components/receive/ReceiveRequestModal";
import { AppToast } from "./components/common/AppToast";
import { useLanTransferApp } from "./hooks/useLanTransferApp";

function App() {
  const app = useLanTransferApp();

  if (!app.snapshot) {
    return (
      <div className="grid min-h-screen place-items-center bg-app-bg text-fg-muted">
        <div className="grid justify-items-center gap-3">
          <Spinner />
          <span>正在启动局域网文件传输...</span>
        </div>
      </div>
    );
  }

  return (
    <AppLayout
        sidebar={
          <LocalStatusPanel
            snapshot={app.snapshot}
            onStartEdit={app.startEditingName}
            onCopyAddress={app.handleCopyAddress}
            onSelectIp={app.handleSelectIp}
            onChooseSaveDir={app.handleChooseSaveDir}
            onOpenSaveDir={app.handleOpenSaveDir}
          />
        }
        center={
          <>
            <TargetConnectionPanel
              snapshot={app.snapshot}
              targetInput={app.targetInput}
              target={app.target}
              targetError={app.targetError}
              targetTesting={app.targetTesting}
              activeSendLocked={app.activeSendLocked}
              onTargetInputChange={app.setTargetInput}
              onConnect={app.handleTestConnection}
              onDeleteRecent={app.handleDeleteRecent}
            />
            <SendFilesPanel
              files={app.files}
              selectedTotal={app.selectedTotal}
              canSend={!!app.target && app.files.length > 0 && !app.activeSendLocked}
              onChooseFiles={app.handleChooseFiles}
              onRemoveFile={app.removeFile}
              onSend={app.handleSend}
            />
          </>
        }
        tasks={
          <TransferTasksPanel
            tasks={app.filteredTasks}
            filter={app.filter}
            onFilterChange={app.setFilter}
            onClearCompleted={app.handleClearCompleted}
          />
        }
        modal={
          <>
            {app.targetTesting && <ConnectingOverlay />}
            <ReceiveRequestModal transfer={app.snapshot.pendingTransfer} onRespond={app.handleRespond} />
            <EditDeviceNameModal
              isOpen={app.editingName}
              value={app.deviceNameDraft}
              onValueChange={app.setDeviceNameDraft}
              onCancel={app.cancelEditingName}
              onSubmit={app.handleSaveDeviceName}
            />
          </>
        }
        toast={<AppToast message={app.busyMessage} onClose={() => app.setBusyMessage(undefined)} />}
    />
  );
}

function ConnectingOverlay() {
  return (
    // 连接测试期间锁定整个界面，避免用户快速切换多个最近设备导致目标状态交错。
    <div className="fixed inset-0 z-[60] grid place-items-center bg-black/45 px-6 backdrop-blur-sm">
      <section
        aria-live="polite"
        className="grid w-full max-w-[320px] justify-items-center gap-3 rounded-2xl border border-border-subtle bg-surface px-6 py-7 shadow-xl"
      >
        <Spinner />
        <div className="text-center">
          <h3 className="m-0 text-[15px] font-semibold text-fg-default">正在连接目标设备</h3>
          <p className="mt-1 mb-0 text-[12px] text-fg-muted">请稍候，正在确认对方服务状态...</p>
        </div>
      </section>
    </div>
  );
}

export default App;
