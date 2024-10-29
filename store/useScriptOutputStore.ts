// /store/useScriptOutputStore.ts
import { create } from "zustand";

interface ScriptOutputState {
  outputs: { [key: string]: string };
  setOutput: (project_id: string, output: string) => void;
  clearOutput: (project_id: string) => void;
}

export const useScriptOutputStore = create<ScriptOutputState>((set) => ({
  outputs: {},
  setOutput: (project_id, output) => {
    console.log(`Setting output for project ${project_id}: ${output}`);
    set((state) => ({
      outputs: {
        ...state.outputs,
        [project_id]: output,
      },
    }));
  },
  clearOutput: (project_id) => {
    console.log(`Clearing output for project ${project_id}`);
    set((state) => ({
      outputs: {
        ...state.outputs,
        [project_id]: "",
      },
    }));
  },
}));
