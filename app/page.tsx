"use client";

import React, { useEffect } from "react";
import useSWR from "swr";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import { invoke } from "@tauri-apps/api/core";
import {
  Calendar,
  Code,
  Globe,
  MoreVertical,
  Plus,
  Search,
} from "lucide-react";
import { useProjectEvents } from "@/hooks/useProjectEvents";
import { toast } from "sonner";

type Project = {
  id: number;
  name: string;
  path: string;
  framework: string;
  description: string;
  lastUpdated: string;
  status: "En cours" | "Terminé" | "En pause";
  packageManager?: string;
  scripts?: { [key: string]: string };
};

// Fonction fetcher pour SWR
const fetchProjects = async (): Promise<Project[]> => {
  try {
    const savedProjects = await invoke<Project[]>("get_projects");

    // Récupération des informations de scripts pour chaque projet
    const projectsWithDetails = await Promise.all(
      savedProjects.map(async (project, index) => {
        const packageInfo = await invoke("get_package_info", {
          path: project.path,
        });
        return {
          ...project,
          id: index + 1,
          framework: project.framework || "Inconnu",
          description: project.description || "Description par défaut",
          lastUpdated:
            project.lastUpdated || new Date().toISOString().split("T")[0],
          status: project.status || "En cours",
          packageManager: packageInfo?.manager,
          scripts: packageInfo?.scripts || {},
        };
      })
    );

    return projectsWithDetails;
  } catch (error) {
    console.error("Erreur lors de la récupération des projets :", error);
    throw error; // Relancer l'erreur pour que SWR puisse la gérer
  }
};

export default function Home() {
  // Utilisation de SWR pour charger les projets
  const { data: projects, error, mutate } = useSWR("projects", fetchProjects);

  // Gestion des erreurs avec useEffect
  useEffect(() => {
    if (error) {
      console.error("Erreur lors du chargement des projets :", error);
      toast.error("Erreur lors du chargement des projets.");
    }
  }, [error]);

  // Fonction pour exécuter un script via Tauri
  const handleRunScript = async (project: Project, scriptName: string) => {
    try {
      await invoke("execute_script", {
        manager: project.packageManager,
        command: scriptName,
        path: project.path,
      });
      toast.success(`Script "${scriptName}" lancé pour ${project.name}`);
    } catch (error) {
      console.error(
        `Erreur lors de l'exécution du script ${scriptName}:`,
        error
      );
      toast.error(`Erreur lors de l'exécution de ${scriptName}`);
    }
  };

  // Fonction pour ouvrir la boîte de dialogue et sélectionner un dossier
  const handleAddFolder = async () => {
    try {
      await invoke("select_folder");
      mutate();
    } catch (error) {
      console.error("Erreur lors de la sélection du dossier :", error);
      toast.error("Erreur lors de la sélection du dossier.");
    }
  };

  // Appel du hook custom pour les événements
  useProjectEvents(projects || []);

  if (!projects) {
    return <p>Chargement des projets...</p>;
  }

  return (
    <div className="flex flex-col h-screen bg-background">
      <header className="flex items-center justify-between p-4 border-b">
        <h1 className="text-2xl font-bold">Gestionnaire de Projets Web</h1>
        <Button onClick={handleAddFolder}>
          <Plus className="mr-2 h-4 w-4" /> Nouveau Projet
        </Button>
      </header>
      <main className="flex-1 p-6 overflow-auto">
        <div className="mb-6 flex items-center space-x-4">
          <div className="relative flex-1">
            <Search className="absolute left-2 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
            <Input
              type="search"
              placeholder="Rechercher des projets..."
              className="pl-8"
            />
          </div>
        </div>
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {projects.map((project) => (
            <div
              key={project.id}
              className="bg-card text-card-foreground rounded-lg shadow-md overflow-hidden"
            >
              <div className="p-4">
                <div className="flex items-center justify-between mb-2">
                  <h3 className="text-lg font-semibold">{project.name}</h3>
                  <Badge variant="secondary">{project.framework}</Badge>
                </div>
                <p className="text-sm text-muted-foreground mb-4">
                  {project.description}
                </p>
                <div className="flex items-center justify-between text-sm text-muted-foreground">
                  <div className="flex items-center">
                    <Calendar className="mr-1 h-4 w-4" />
                    {project.lastUpdated}
                  </div>
                  <div className="flex items-center">
                    <div className="w-2 h-2 rounded-full mr-1" />
                    {project.status}
                  </div>
                </div>
              </div>
              <div className="bg-muted p-4 flex justify-between items-center">
                <Button variant="outline" size="sm">
                  <Globe className="mr-2 h-4 w-4" />
                  Voir le site
                </Button>
                <Button variant="outline" size="sm">
                  <Code className="mr-2 h-4 w-4" />
                  Code source
                </Button>
                <DropdownMenu>
                  <DropdownMenuTrigger asChild>
                    <Button variant="ghost" size="icon">
                      <MoreVertical className="h-4 w-4" />
                    </Button>
                  </DropdownMenuTrigger>
                  <DropdownMenuContent align="end">
                    <DropdownMenuItem>Modifier</DropdownMenuItem>
                    <DropdownMenuItem>Supprimer</DropdownMenuItem>
                    {Object.entries(project.scripts || {}).map(
                      ([scriptName]) => (
                        <DropdownMenuItem
                          key={scriptName}
                          onClick={() => handleRunScript(project, scriptName)}
                        >
                          Exécuter {scriptName}
                        </DropdownMenuItem>
                      )
                    )}
                  </DropdownMenuContent>
                </DropdownMenu>
              </div>
            </div>
          ))}
        </div>
      </main>
    </div>
  );
}
