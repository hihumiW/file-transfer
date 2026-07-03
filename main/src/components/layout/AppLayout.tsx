import type { ReactNode } from "react";

type AppLayoutProps = {
  sidebar: ReactNode;
  center: ReactNode;
  tasks: ReactNode;
  modal?: ReactNode;
  toast?: ReactNode;
};

export function AppLayout({ sidebar, center, tasks, modal, toast }: AppLayoutProps) {
  return (
    <main className="min-h-screen bg-app-bg p-5 max-[899px]:p-3">
      <section
        className={[
          "grid min-h-[calc(100vh-40px)] gap-3",
          "grid-cols-[minmax(300px,3fr)_minmax(420px,4fr)_minmax(300px,3fr)]",
          "max-[1279px]:grid-cols-[minmax(300px,3fr)_minmax(380px,4fr)_minmax(300px,3fr)] max-[1279px]:gap-2.5",
          "max-[1023px]:min-h-0 max-[1023px]:grid-cols-[260px_minmax(420px,1fr)]",
          "max-[1023px]:[grid-template-areas:'sidebar_main'_'sidebar_tasks']",
          "max-[899px]:grid-cols-1 max-[899px]:[grid-template-areas:none]",
        ].join(" ")}
      >
        <aside className="min-w-0 max-[1023px]:[grid-area:sidebar] max-[899px]:[grid-area:auto]">{sidebar}</aside>
        <section className="grid min-w-0 grid-rows-[auto_minmax(0,1fr)] gap-3 max-[1023px]:[grid-area:main] max-[899px]:[grid-area:auto]">
          {center}
        </section>
        <aside className="min-w-0 max-[1023px]:[grid-area:tasks] max-[899px]:[grid-area:auto]">{tasks}</aside>
      </section>
      {modal}
      {toast}
    </main>
  );
}
