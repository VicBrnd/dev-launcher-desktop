// /app/page.tsx
"use client";

import { ThemeButton } from "@/components/theme-button";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import { Switch } from "@/components/ui/switch";
import { useFetchProjects } from "@/hooks/useFetchProjects";
import { useScriptListener } from "@/hooks/useScriptListener";
import { Project } from "@/schemas/schemas";
import { useScriptOutputStore } from "@/store/useScriptOutputStore";
import { invoke } from "@tauri-apps/api/core";
import {
  CircleX,
  Code,
  FilePenLine,
  MoreVertical,
  PackagePlus,
  Pencil,
  Search,
} from "lucide-react";
import React, { useState } from "react";
import { toast } from "sonner";

const Home: React.FC = () => {
  const { data: projects, error, mutate } = useFetchProjects();
  const scriptOutputs = useScriptOutputStore((state) => state.outputs);
  const [searchTerm, setSearchTerm] = useState<string>("");

  useScriptListener();

  console.log("Projets dans Home:", projects); // Log ajouté

  const handleRunScript = async (project: Project, scriptName: string) => {
    try {
      await invoke("execute_script", {
        manager: project.package_manager,
        command: scriptName,
        path: project.path,
        project_id: project.id,
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

  const handleAddFolder = async () => {
    try {
      await invoke("select_folder");
      mutate();
    } catch (error) {
      console.error("Erreur lors de la sélection du dossier :", error);
      toast.error("Erreur lors de la sélection du dossier.");
    }
  };

  const handleFrameworkUrl = (framework_url?: string) => {
    if (framework_url) {
      window.open(framework_url, "_blank", "noopener,noreferrer");
    } else {
      toast.error("Aucune URL disponible pour ce framework.");
    }
  };

  const handleDeleteProject = async (projectId: string) => {
    try {
      await invoke("delete_project", { id: projectId }); // Commande Rust pour supprimer le projet
      mutate(); // Rafraîchir les projets après suppression
      toast.success("Projet supprimé avec succès.");
    } catch (error) {
      console.error("Erreur lors de la suppression du projet :", error);
      toast.error("Erreur lors de la suppression du projet.");
    }
  };

  if (error) {
    return <p>Erreur lors du chargement des projets.</p>;
  }

  if (!projects) {
    return <p>Chargement des projets...</p>;
  }

  const filteredProjects = projects.filter((project) =>
    project.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  return (
    <div className="flex h-screen bg-background">
      <div className="flex-1 flex flex-col overflow-hidden">
        <header className="flex items-center bg-muted justify-between p-4 text-card-foreground shadow-sm border z-20 m-4 rounded-lg h-12">
          <div className="flex items-center">
            <h1 className="text-xl font-bold">Dev Launcher</h1>
          </div>
          <div className="flex items-center space-x-2">
            <ThemeButton />
            <Button onClick={handleAddFolder} variant="ghost" size="icon">
              <PackagePlus className="h-7 w-7" />
            </Button>
          </div>
        </header>
        <main className="flex-1 p-4 overflow-auto">
          <div className="mb-6">
            <div className="relative">
              <Search className="absolute left-2 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
              <Input
                type="search"
                placeholder="Rechercher des projets..."
                className="pl-8"
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
              />
            </div>
          </div>
          <div className="grid gap-4 grid-cols-1 sm:grid-cols-2 lg:grid-cols-3">
            {filteredProjects.map((project) => (
              <Card
                key={project.id}
                className="flex flex-col justify-between transition-all duration-300"
              >
                <CardHeader className="p-4">
                  <span className="truncate mr-2">{project.id}</span>
                  <CardTitle className="text-lg flex items-center justify-between">
                    <span className="truncate mr-2">{project.name}</span>
                    {project.framework_url ? (
                      <a
                        href={project.framework_url}
                        target="_blank"
                        rel="noopener noreferrer"
                      >
                        <Badge variant="secondary" className="cursor-pointer">
                          {project.framework || "Inconnu"}
                        </Badge>
                      </a>
                    ) : (
                      <Badge
                        variant="secondary"
                        className="cursor-default"
                        onClick={() =>
                          handleFrameworkUrl(project.framework_url)
                        }
                      >
                        {project.framework || "Inconnu"}
                      </Badge>
                    )}
                  </CardTitle>
                </CardHeader>
                <CardContent className="p-4 pt-0">
                  <div className="flex items-center justify-between text-xs text-muted-foreground mt-2">
                    <div className="flex items-center">
                      <p className="text-sm text-muted-foreground overflow-hidden text-ellipsis whitespace-nowrap">
                        {scriptOutputs[project.id]
                          ? scriptOutputs[project.id]
                          : project.description ||
                            "Aucune description disponible."}
                      </p>
                    </div>
                    {/* Switch */}
                    <div className="flex items-center space-x-2">
                      <Switch id={`switch-${project.id}`} />
                    </div>
                  </div>
                </CardContent>
                <CardFooter className="bg-muted p-2 flex justify-between items-center rounded-b-lg">
                  <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                      <Button variant="outline" size="sm">
                        <Code className="h-4 w-4 mr-1" />
                        Script
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="start">
                      {project.scripts &&
                      Object.keys(project.scripts).length > 0 ? (
                        Object.entries(project.scripts).map(([scriptName]) => (
                          <DropdownMenuItem
                            key={scriptName}
                            onClick={() => handleRunScript(project, scriptName)}
                          >
                            Exécuter {scriptName}
                          </DropdownMenuItem>
                        ))
                      ) : (
                        <DropdownMenuItem disabled>
                          Aucun script disponible
                        </DropdownMenuItem>
                      )}
                    </DropdownMenuContent>
                  </DropdownMenu>
                  <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                      <Button variant="outline" size="sm">
                        <MoreVertical className="h-4 w-4" />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                      <DropdownMenuItem>
                        <FilePenLine className="h-4 w-4 mr-1" />
                        Open in Editor
                      </DropdownMenuItem>
                      <DropdownMenuItem>
                        <Pencil className="h-4 w-4 mr-1" />
                        Modifier
                      </DropdownMenuItem>
                      <DropdownMenuItem
                        onClick={() => handleDeleteProject(project.id)}
                      >
                        <CircleX className="h-4 w-4 mr-1" />
                        Supprimer
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </CardFooter>
              </Card>
            ))}
          </div>
        </main>
      </div>
    </div>
  );
};

export default Home;
