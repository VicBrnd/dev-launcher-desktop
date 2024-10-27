import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { toast } from "sonner";
import { Project } from "@/types"; // Ajustez l'import du type si nécessaire
import { mutate } from "swr";

export function useProjectEvents(projects: Project[]) {
  useEffect(() => {
    // Abonnement aux événements Tauri
    const unlistenFolderSuccess = listen<string>("folder_success", (event) => {
      const newProjectData = JSON.parse(event.payload);

      // Vérification des doublons
      const isDuplicate = projects.some(
        (project) => project.path === newProjectData.path
      );
      if (isDuplicate) {
        toast.error("Ce dossier est déjà ajouté en tant que projet.");
        return;
      }

      // Mettre à jour le cache SWR pour rafraîchir la liste des projets
      mutate(
        "projects",
        [
          ...projects,
          {
            ...newProjectData,
            id: projects.length + 1,
            description: newProjectData.description || "Description par défaut",
            status: "En cours",
          },
        ],
        false
      );

      toast.success(`Le projet "${newProjectData.name}" a été ajouté.`);
    });

    const unlistenFolderError = listen<string>("folder_error", (event) => {
      toast.error(event.payload);
    });

    return () => {
      // Nettoyage des écouteurs d'événements
      unlistenFolderSuccess.then((unlisten) => unlisten());
      unlistenFolderError.then((unlisten) => unlisten());
    };
  }, [projects]);
}
