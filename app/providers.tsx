"use client";

import { ThemeProvider } from "next-themes";
import type { PropsWithChildren } from "react";

export function Providers({ children }: PropsWithChildren) {
  return (
    <ThemeProvider
      attribute="class"
      defaultTheme="system"
      enableSystem
      disableTransitionOnChange
    >
      {/* <ContextMenuDemo> */}
      {children}
      {/* </ContextMenuDemo> */}
    </ThemeProvider>
  );
}
