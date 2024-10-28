// src/hooks/useFetchProjects.ts

import { Project, ProjectSchema } from "@/schemas/schemas";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";
import useSWR from "swr";

// Fonction de récupération des projets
const fetcher = async (): Promise<Project[]> => {
  try {
    // Récupère les projets depuis le backend
    const projectsRaw: unknown = await invoke("get_projects");

    console.log("Projets reçus du backend:", projectsRaw); // Pour débogage

    // Valide et parse les projets avec Zod
    const projects = ProjectSchema.array().parse(projectsRaw);
    return projects;
  } catch (error) {
    console.error("Erreur lors de la récupération des projets :", error);
    throw error;
  }
};

// Hook personnalisé pour récupérer les projets avec SWR
export const useFetchProjects = () => {
  return useSWR<Project[]>("projects", fetcher, {
    revalidateOnFocus: false,
    onError: (error) => {
      console.error("Erreur lors du chargement des projets :", error);
      toast.error("Erreur lors du chargement des projets.");
    },
  });
};
