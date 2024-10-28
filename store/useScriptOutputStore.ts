// src/store/useScriptOutputStore.ts
import { create } from "zustand";

interface ScriptOutputState {
  outputs: { [key: number]: string[] };
  setOutput: (projectId: number, output: string) => void;
  clearOutputs: (projectId: number) => void;
}

export const useScriptOutputStore = create<ScriptOutputState>((set) => ({
  outputs: {},
  setOutput: (projectId, output) => {
    set((state) => ({
      outputs: {
        ...state.outputs,
        [projectId]: state.outputs[projectId]
          ? [...state.outputs[projectId], output]
          : [output],
      },
    }));
  },
  clearOutputs: (projectId) => {
    set((state) => ({
      outputs: {
        ...state.outputs,
        [projectId]: [],
      },
    }));
  },
}));
