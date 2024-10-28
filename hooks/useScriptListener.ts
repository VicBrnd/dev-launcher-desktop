// src/hooks/useScriptListener.ts
import { useScriptOutputStore } from "@/store/useScriptOutputStore";
import { listen } from "@tauri-apps/api/event";
import { useEffect } from "react";

interface ScriptEvent {
  project_id: string;
  output: string;
}

export const useScriptListener = () => {
  const setOutput = useScriptOutputStore((state) => state.setOutput);

  useEffect(() => {
    const handleEvent = (event: { payload: ScriptEvent }) => {
      const { project_id, output } = event.payload;
      setOutput(project_id, output);
    };

    const unlistenOutput = listen<ScriptEvent>("script_output", handleEvent);
    const unlistenError = listen<ScriptEvent>("script_error", handleEvent);

    return () => {
      unlistenOutput.then((unlisten) => unlisten());
      unlistenError.then((unlisten) => unlisten());
    };
  }, [setOutput]);
};
