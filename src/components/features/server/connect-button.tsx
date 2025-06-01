import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Button } from '@/components/ui/button';
import { toast } from 'sonner';
import { IconServer } from '@tabler/icons-react';
import { PasswordDialog } from './password-dialog';
import { Server } from '@/types';
import { useServerContext, ConnectionStatuses } from "@/components/providers/server.tsx";

interface ConnectButtonProps {
  server: Server;
  onSuccess?: () => void;
  onError?: (error: string) => void;
  children?: React.ReactNode;
  variant?: 'default' | 'destructive' | 'outline' | 'secondary' | 'ghost' | 'link';
}

export function ConnectButton({
  server,
  onSuccess,
  onError,
  children,
  variant = 'default',
}: ConnectButtonProps) {
  const { setConnectionStatus, connectionStatus, connectedServer, setConnectedServer, setConnectionError } = useServerContext();
  const [showPasswordDialog, setShowPasswordDialog] = useState(false);
  const [passwordError, setPasswordError] = useState<string | null>(null);

  const handleConnect = async () => {
    try {
      if (connectionStatus === ConnectionStatuses.connected) {
        await invoke('disconnect_from_server');
        setConnectionStatus(ConnectionStatuses.disconnected);
        setConnectedServer(null);

        if (connectedServer?.id === server.id) {
          toast.success(`Disconnected from ${server.name}`);
          return;
        }
      }

      setConnectionStatus(ConnectionStatuses.connecting);
      setPasswordError(null);

      if (onSuccess) onSuccess();

      await invoke('try_connect_to_server', { id: server.id });

      toast.success(`Connected to ${server.name}`);

      setConnectionStatus(ConnectionStatuses.connected);
      setConnectedServer(server);
      if (onSuccess) onSuccess();
    } catch (error) {
      toast.error('SSH key connection failed: {error}');

      setShowPasswordDialog(true);

      setConnectionStatus(ConnectionStatuses.disconnected);

      setConnectionError(String(error));

      if (onError) onError(String(error));
    }
  };

  const handlePasswordSubmit = async (password: string) => {
    try {
      setConnectionStatus(ConnectionStatuses.connecting);

      await invoke('connect_with_password', {
        id: server.id, 
        password 
      });

      setShowPasswordDialog(false);

      toast.success(`Connected to ${server.name}`);

      setConnectionStatus(ConnectionStatuses.connecting);
      setConnectedServer(server);

      if (onSuccess) onSuccess();
    } catch (error) {
      console.error('Password connection failed:', error);
      setPasswordError(`Failed to connect: ${error}`);
      setConnectionStatus(ConnectionStatuses.disconnected);
      if (onError) onError(String(error));
    }
  };

  const handlePasswordCancel = () => {
    setShowPasswordDialog(false);
    setPasswordError(null);
    setConnectionError(null);
  };

  return (
    <>
      <Button 
        variant={variant} 
        onClick={handleConnect} 
        disabled={(connectionStatus === ConnectionStatuses.connecting)}
      >
        {children || (
          <>
            <IconServer className="mr-2 h-4 w-4" />
            {connectionStatus === ConnectionStatuses.connecting? 'Connecting...' :
            connectionStatus === ConnectionStatuses.connected? 'Disconnect' : 'Connect'}
          </>
        )}
      </Button>

      <PasswordDialog
        open={showPasswordDialog}
        onOpenChange={setShowPasswordDialog}
        serverName={server.name}
        onSubmit={handlePasswordSubmit}
        onCancel={handlePasswordCancel}
        isLoading={(connectionStatus === ConnectionStatuses.connecting)}
        error={passwordError}
      />
    </>
  );
}
