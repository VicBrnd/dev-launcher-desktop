// src/store/useScriptOutputStore.ts
import { create } from "zustand";

interface ScriptOutputState {
  outputs: { [key: string]: string };
  setOutput: (projectId: string, output: string) => void;
  clearOutput: (projectId: string) => void;
}

export const useScriptOutputStore = create<ScriptOutputState>((set) => ({
  outputs: {},
  setOutput: (projectId, output) => {
    console.log(`Setting output for project ${projectId}: ${output}`);
    set((state) => ({
      outputs: {
        ...state.outputs,
        [projectId]: output,
      },
    }));
  },
  clearOutput: (projectId) => {
    console.log(`Clearing output for project ${projectId}`);
    set((state) => ({
      outputs: {
        ...state.outputs,
        [projectId]: "",
      },
    }));
  },
}));
