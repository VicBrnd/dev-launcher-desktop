"use client";

import { Moon, Sun } from "lucide-react";
import { useTheme } from "next-themes";
import { ReactElement } from "react";
import { Button } from "./ui/button";

export function ThemeButton(): ReactElement {
  const { setTheme, theme } = useTheme();
  const toggleTheme = () => {
    setTheme(theme === "light" ? "dark" : "light");
  };

  return (
    <Button
      onClick={toggleTheme}
      variant="ghost"
      size="icon"
      className="h-7 w-7"
    >
      <div className="relative size-6">
        <Sun
          className="absolute inset-0 m-auto size-5 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
          aria-hidden="true"
        />
        <Moon
          className="absolute inset-0 m-auto size-5 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
          aria-hidden="true"
        />
      </div>
    </Button>
  );
}
