// /store/useScriptOutputStore.ts
import { create } from "zustand";

interface ScriptOutputState {
  outputs: { [key: string]: string };
  setOutput: (id: string, output: string) => void;
  clearOutput: (id: string) => void;
}

export const useScriptOutputStore = create<ScriptOutputState>((set) => ({
  outputs: {},
  setOutput: (id, output) => {
    console.log(`Setting output for project ${id}: ${output}`);
    set((state) => ({
      outputs: {
        ...state.outputs,
        [id]: output,
      },
    }));
  },
  clearOutput: (id) => {
    console.log(`Clearing output for project ${id}`);
    set((state) => ({
      outputs: {
        ...state.outputs,
        [id]: "",
      },
    }));
  },
}));
