// src/store/useScriptOutputStore.ts
import { create } from "zustand";

interface ScriptOutputState {
  outputs: { [key: string]: string }; // Utilisation de string pour les UUIDs
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
        [projectId]: output, // Remplace la sortie précédente par la nouvelle
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
