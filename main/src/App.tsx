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

export default App;
