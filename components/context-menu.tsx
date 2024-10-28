import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/components/ui/context-menu";
import { FolderPlus, Monitor, RefreshCw } from "lucide-react";
import { PropsWithChildren } from "react";

export function ContextMenuDemo({ children }: PropsWithChildren) {
  const handleReload = () => {
    window.location.reload(); // Recharge la page actuelle
  };

  const handleInspect = () => {
    alert("Cmd + Option + I");
  };

  return (
    <ContextMenu>
      <ContextMenuTrigger>{children}</ContextMenuTrigger>
      <ContextMenuContent>
        {/* Option pour créer un nouveau projet */}
        <ContextMenuItem>
          <FolderPlus className="mr-2 h-4 w-4" />
          Nouveau projet
        </ContextMenuItem>

        {/* Option pour recharger la page */}
        <ContextMenuItem onClick={handleReload}>
          <RefreshCw className="mr-2 h-4 w-4" />
          Recharger la page
        </ContextMenuItem>

        {/* Option pour "Inspecter" */}
        <ContextMenuItem onClick={handleInspect}>
          <Monitor className="mr-2 h-4 w-4" />
          Inspecter
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}
