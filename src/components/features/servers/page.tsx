import { invoke } from '@tauri-apps/api/core';
import { useState } from 'react';
import { usePageContext } from '@/components/providers/page';
import {ConnectionStatuses, useServerContext} from '@/components/providers/server';
import { useGlobalState } from '@/components/providers/global-state.tsx';
import { ConnectButton } from '@/components/features/server/connect-button';
import { Button } from '@/components/ui/button';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { MoreHorizontal, EyeIcon, EyeOffIcon } from "lucide-react";
import {toast} from "sonner";
import { EditServerDialog } from '@/components/features/server/edit-server-dialog';

export default function ServersPage(){
    const { servers, loading, error, visibleIpMap, toggleIpVisibility, fetchServers } = useGlobalState();
    const { setCurrentPage } = usePageContext();
    const { connectionStatus, connectedServer, setConnectionStatus, setConnectedServer } = useServerContext();
    const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
    const [serverToDelete, setServerToDelete] = useState<number | null>(null);
    const [deleteError, setDeleteError] = useState<string | null>(null);
    const [editDialogOpen, setEditDialogOpen] = useState(false);
    const [serverToEdit, setServerToEdit] = useState<number | null>(null);

    const handleManageSettings = (serverId: number) => {
        setServerToEdit(serverId);
        setEditDialogOpen(true);
    };

    const handleDeleteClick = (serverId: number) => {
        setServerToDelete(serverId);
        setDeleteError(null);
        setDeleteDialogOpen(true);
    };

    const handleDeleteConfirm = async () => {
        if (serverToDelete === null) return;

        try {
            await invoke('delete_server', { id: serverToDelete });
            await fetchServers();

            setDeleteDialogOpen(false);
            setServerToDelete(null);
            setDeleteError(null);
            toast.success('Server deleted successfully.');
        } catch (err) {
            console.error('Failed to delete server:', err);
            setDeleteError(err instanceof Error ? err.message : 'Failed to delete server. Please try again.');
        }
    };

    if (loading) {
        return <div className="flex justify-center items-center h-full">Loading servers...</div>;
    }

    if (error) {
        return (
            <div className="flex flex-col justify-center items-center h-full gap-4">
                <p className="text-red-500">{error}</p>
                <Button onClick={() => window.location.reload()}>Retry</Button>
            </div>
        );
    }

    if (servers.length === 0) {
        return (
            <div className="flex flex-col justify-center items-center h-full gap-4">
                <p>No servers found. Add a server to get started.</p>
            </div>
        );
    }

    const serverToDeleteData = serverToDelete !== null ? servers.find(s => s.id === serverToDelete) : null;

    return (
        <div className="container mx-auto py-4">
            <Table>
                <TableHeader className="bg-accent text-accent-foreground">
                    <TableRow>
                        <TableHead>Name</TableHead>
                        <TableHead>Hostname</TableHead>
                        <TableHead>IP Address</TableHead>
                        <TableHead>Port</TableHead>
                        <TableHead>Username</TableHead>
                        <TableHead>Status</TableHead>
                        <TableHead className="text-right">Actions</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {servers.map((server) => (
                        <TableRow key={server.id}>
                            <TableCell className="font-medium">{server.name}</TableCell>
                            <TableCell>{server.hostname}</TableCell>
                            <TableCell>
                                <div className="flex items-center gap-2">
                                    <span>{server.ip_address ? (visibleIpMap[server.id || 0] ? server.ip_address : '••••••••••') : 'N/A'}</span>
                                    {server.ip_address && (
                                        <Button
                                            variant="ghost"
                                            size="icon"
                                            className="h-6 w-6 p-0"
                                            onClick={(e) => {
                                                e.stopPropagation();
                                                server.id && toggleIpVisibility(server.id);
                                            }}
                                            title={visibleIpMap[server.id || 0] ? "Hide IP Address" : "Show IP Address"}
                                        >
                                            {visibleIpMap[server.id || 0] ?
                                                <EyeOffIcon className="h-3 w-3" /> :
                                                <EyeIcon className="h-3 w-3" />
                                            }
                                        </Button>
                                    )}
                                </div>
                            </TableCell>
                            <TableCell>{server.port}</TableCell>
                            <TableCell>{server.username}</TableCell>
                            <TableCell>
                                <div className="flex items-center">
                                    <span className={`inline-block w-3 h-3 rounded-full mr-2 ${
                                        connectedServer?.id === server.id && connectionStatus === 'connected' ? 'bg-green-500' : 
                                        connectedServer?.id === server.id && connectionStatus === 'connecting' ? 'bg-yellow-500' : 'bg-red-500'
                                    }`}></span>
                                    <span>
                                        {connectedServer?.id === server.id && connectionStatus === 'connected' ? 'Connected' :
                                         connectedServer?.id === server.id && connectionStatus === 'connecting' ? 'Connecting...' : 'Disconnected'}
                                    </span>
                                </div>
                            </TableCell>
                            <TableCell className="text-right">
                                <div className="flex justify-end gap-2">
                                    <ConnectButton
                                        server={server}
                                        variant="default"
                                    />
                                    <DropdownMenu>
                                        <DropdownMenuTrigger asChild>
                                            <Button variant="ghost" size="sm">
                                                <MoreHorizontal className="h-4 w-4" />
                                                <span className="sr-only">Actions</span>
                                            </Button>
                                        </DropdownMenuTrigger>
                                        <DropdownMenuContent align="end">
                                            <DropdownMenuItem 
                                                onClick={() => server.id && handleManageSettings(server.id)}
                                            >
                                                Edit
                                            </DropdownMenuItem>
                                            <DropdownMenuItem 
                                                onClick={() => server.id && handleDeleteClick(server.id)}
                                                className="text-red-600"
                                            >
                                                Delete
                                            </DropdownMenuItem>
                                        </DropdownMenuContent>
                                    </DropdownMenu>
                                </div>
                            </TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>

            <Dialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle>Delete Server</DialogTitle>
                        <DialogDescription>
                            Are you sure you want to delete the server "{serverToDeleteData?.name}"? 
                            This action cannot be undone.
                        </DialogDescription>
                    </DialogHeader>

                    {deleteError && (
                        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative mb-4">
                            <strong className="font-bold">Error: </strong>
                            <span className="block sm:inline">{deleteError}</span>
                        </div>
                    )}

                    <DialogFooter>
                        <Button 
                            variant="outline" 
                            onClick={() => setDeleteDialogOpen(false)}
                        >
                            Cancel
                        </Button>
                        <Button 
                            variant="destructive" 
                            onClick={handleDeleteConfirm}
                        >
                            Delete
                        </Button>
                    </DialogFooter>
                </DialogContent>
            </Dialog>

            {serverToEdit !== null && (
                <EditServerDialog
                    open={editDialogOpen}
                    onOpenChange={setEditDialogOpen}
                    server={servers.find(s => s.id === serverToEdit) || servers[0]}
                    onSuccess={() => {
                        setServerToEdit(null);
                        fetchServers();
                    }}
                />
            )}
        </div>
    );
}
