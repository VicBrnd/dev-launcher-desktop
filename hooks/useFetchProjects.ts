import { PackageInfoSchema, Project, ProjectSchema } from "@/schemas/schemas";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useCallback, useEffect } from "react";
import { toast } from "sonner";
import useSWR, { mutate } from "swr";

// Validateur et fetcher de projets
const fetcher = async (): Promise<Project[]> => {
  const projectsRaw = await invoke("fetch_projects");
  const parsedProjects = ProjectSchema.array().safeParse(projectsRaw);

  if (!parsedProjects.success) throw new Error("Données de projets invalides.");

  return Promise.all(
    parsedProjects.data.map(async (project, index) => {
      const packageInfoRaw = await invoke("fetch_package_json", {
        path: project.path,
      });
      const parsedPackage = PackageInfoSchema.safeParse(packageInfoRaw);

      if (!parsedPackage.success)
        throw new Error(`Package invalide pour ${project.name}`);

      return {
        ...project,
        id: project.id || (index + 1).toString(),
        package_manager: parsedPackage.data.manager,
        scripts: parsedPackage.data.scripts,
      };
    })
  );
};

// Gestion de l'ajout de projet dans le cache SWR
const addProjectToCache = async (newProject: Project) => {
  mutate(
    "projects",
    (projects: Project[] = []) => [...projects, newProject],
    false
  );
  await mutate("projects"); // Forcer la revalidation des projets
};

// Hook personnalisé
export const useFetchProjects = () => {
  const handleEvent = useCallback(
    (event: { payload: unknown }, isError: boolean) => {
      try {
        const data = JSON.parse(event.payload as string); // Assurer que payload est une chaîne
        if (isError) {
          toast.error(data);
        } else {
          addProjectToCache(data); // Ajout et revalidation du cache
          toast.success("Dossier ajouté avec succès.");
        }
      } catch {
        toast.error("Erreur lors de l'ajout du dossier.");
      }
    },
    []
  );

  useEffect(() => {
    const unlistenSuccess = listen("folder_success", (e) =>
      handleEvent(e, false)
    );
    const unlistenError = listen("folder_error", (e) => handleEvent(e, true));
    return () => {
      unlistenSuccess.then((u) => u());
      unlistenError.then((u) => u());
    };
  }, [handleEvent]);

  return useSWR<Project[]>("projects", fetcher, {
    revalidateOnFocus: false,
    onError: () => toast.error("Erreur lors du chargement des projets."), // Erreur supprimée
  });
};
