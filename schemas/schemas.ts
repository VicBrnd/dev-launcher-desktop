// src/schemas/schemas.ts
import { z } from "zod";

// Schéma pour les informations du package
export const PackageInfoSchema = z.object({
  manager: z.string().optional(),
  scripts: z.record(z.string()).optional(),
});

export type PackageInfo = z.infer<typeof PackageInfoSchema>;

// Schéma de base pour le projet
export const ProjectSchema = z.object({
  id: z.string(), // UUID unique
  name: z.string(),
  path: z.string(),
  framework: z.string().optional(),
  framework_url: z.string().url().optional(),
  description: z.string().optional(),
  status: z.string().optional(),
  package_manager: z.string().optional(),
  scripts: z.record(z.string()).optional(),
});

export type Project = z.infer<typeof ProjectSchema>;
