import React, { createContext, useContext, useState, useEffect } from 'react';
import {Server, ServerConnectionStatus} from "@/types";
import { usePageContext } from './page';

interface ServerContextType {
  connectionStatus: ServerConnectionStatus;
  setConnectionStatus: (status: ServerConnectionStatus) => void;
  connectedServer: Server | null;
  setConnectedServer: (server: Server | null) => void;
  connectionError: string | null;
  setConnectionError: (error: string | null) => void;
}

const ServerContext = createContext<ServerContextType | undefined>(undefined);

export const ConnectionStatuses: { [T in ServerConnectionStatus]: T} = {
    connected: 'connected',
    disconnected: "disconnected",
    connecting: 'connecting'
};

export function ServerProvider({ children }: { children: React.ReactNode }) {
  const { setCurrentPage } = usePageContext();
  const [connectionStatus, setConnectionStatus] = useState<ServerConnectionStatus>(ConnectionStatuses.disconnected);
  const [connectedServer, setConnectedServer] = useState<Server | null>(null);
  const [connectionError, setConnectionError] = useState<string | null>(null);

  useEffect(() => {
    if (connectionStatus === ConnectionStatuses.connected && connectedServer) {
      setCurrentPage('server');
    }
  }, [connectedServer]);

  return (
    <ServerContext.Provider 
      value={{
          connectionStatus,
          connectedServer,
          connectionError,

          setConnectionStatus,
          setConnectedServer,
          setConnectionError,
      }}
    >
      {children}
    </ServerContext.Provider>
  );
}

export function useServerContext() {
  const context = useContext(ServerContext);

  if (context === undefined) {
    throw new Error('useServerContext must be used within a ServerProvider');
  }

  return context;
}
