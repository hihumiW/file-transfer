import { Button } from "@heroui/react";

export function AppToast({ message, onClose }: { message?: string; onClose: () => void }) {
  if (!message) return null;

  return (
    <Button
      variant="primary"
      className="fixed bottom-5 right-5 z-50 max-w-[480px] justify-start bg-fg-toast px-4 py-3 text-left text-white shadow-[0_12px_40px_rgba(20,32,50,0.22)]"
      onPress={onClose}
    >
      {message}
    </Button>
  );
}
