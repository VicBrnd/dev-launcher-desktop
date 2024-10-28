// src/hooks/useFetchProjects.ts
import {
  PackageInfo,
  PackageInfoSchema,
  Project,
  ProjectSchema,
} from "@/schemas/schemas"; // Assurez-vous que le chemin est correct
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";
import useSWR from "swr";

const fetcher = async (): Promise<Project[]> => {
  const savedProjects = await invoke<Project[]>("get_projects");

  const projectsWithDetails = await Promise.all(
    savedProjects.map(async (project, index) => {
      const packageInfoRaw = await invoke<PackageInfo>("get_package_info", {
        path: project.path,
      });

      // Validation avec Zod
      const parsedProject = ProjectSchema.parse(project);
      const parsedPackageInfo = PackageInfoSchema.parse(packageInfoRaw);

      return {
        ...parsedProject,
        id: index + 1,
        framework: parsedProject.framework || "NaN",
        description: parsedProject.description,
        status: parsedProject.status,
        packageManager: parsedPackageInfo.manager,
        scripts: parsedPackageInfo.scripts,
      };
    })
  );

  return projectsWithDetails;
};

export const useFetchProjects = () => {
  return useSWR<Project[]>("projects", fetcher, {
    revalidateOnFocus: false,
    onError: (error) => {
      console.error("Erreur lors du chargement des projets :", error);
      toast.error("Erreur lors du chargement des projets.");
    },
  });
};
