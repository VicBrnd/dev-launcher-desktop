// /hooks/useFetchProjects.ts

import { Project, ProjectSchema } from "@/schemas/schemas";
import { invoke } from "@tauri-apps/api/core"; // Correction de l'import
import { listen } from "@tauri-apps/api/event";
import { useEffect } from "react";
import { toast } from "sonner";
import useSWR, { mutate } from "swr";

// Fonction de récupération des projets
const fetcher = async (): Promise<Project[]> => {
  try {
    // Récupère les projets depuis le backend
    const projectsRaw: unknown = await invoke("get_projects");

    console.log("Projets:", projectsRaw); // Pour débogage

    // Valide et parse les projets avec Zod
    const projects = ProjectSchema.array().parse(projectsRaw);
    return projects;
  } catch (error) {
    console.error("Erreur lors de la récupération des projets :", error);
    throw error;
  }
};

// Hook personnalisé pour récupérer les projets avec SWR et écouter les événements
export const useFetchProjects = () => {
  useEffect(() => {
    // Écoute de l'événement de succès d'ajout de dossier
    const unlistenSuccess = listen("folder_success", (event) => {
      try {
        const newProject: Project = JSON.parse(event.payload as string);
        // Met à jour le cache SWR en ajoutant le nouveau projet
        mutate(
          "projects",
          (currentProjects?: Project[]) => {
            if (currentProjects) {
              return [...currentProjects, newProject];
            }
            return [newProject];
          },
          false
        ); // false pour ne pas revalider immédiatement
        toast.success("Dossier ajouté avec succès.");
      } catch (error) {
        console.error("Erreur lors de l'analyse du payload:", error);
        toast.error("Erreur lors de l'ajout du dossier.");
      }
    });

    // Écoute de l'événement d'erreur d'ajout de dossier
    const unlistenError = listen("folder_error", (event) => {
      const errorMessage: string = event.payload as string;
      console.error("Erreur lors de l'ajout du dossier:", errorMessage);
      toast.error(errorMessage);
    });

    // Nettoyage des écouteurs lors du démontage
    return () => {
      unlistenSuccess.then((f) => f()).catch((e) => console.error(e));
      unlistenError.then((f) => f()).catch((e) => console.error(e));
    };
  }, []);

  return useSWR<Project[]>("projects", fetcher, {
    revalidateOnFocus: false,
    onError: (error) => {
      toast.error("Erreur lors du chargement des projets.", error);
    },
  });
};
