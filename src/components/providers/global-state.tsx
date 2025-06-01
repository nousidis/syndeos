import React, {createContext, useContext, useState, useEffect} from 'react';
import {invoke} from "@tauri-apps/api/core";
import {Servers} from "@/types.ts";

interface GlobalStateContextType {
  servers: Servers;
  fetchServers: () => Promise<void>;
  setServers: (servers: Servers) => void;
  loading: boolean;
  error: string | null;
  toggleIpVisibility: (serverId: number) => void;
  visibleIpMap: Record<number, boolean>;
}

const GlobalStateContext = createContext<GlobalStateContextType | undefined>(undefined);

export function GlobalStateProvider({ children }: { children: React.ReactNode }) {
  const [servers, setServers] = useState<Servers>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [visibleIpMap, setVisibleIpMap] = useState<Record<number, boolean>>({});

  const toggleIpVisibility = (serverId: number) => {
    setVisibleIpMap(prev => ({
      ...prev,
      [serverId]: !prev[serverId]
    }));
  };

  const fetchServers = async () => {
    try {
      setLoading(true);

      const result = await invoke<Servers>('get_servers');

      setServers(result);

      setError(null);
    } catch (err) {
      console.error('Failed to fetch servers:', err);
      setError('Failed to load servers. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchServers();
  }, []);

  return (
    <GlobalStateContext.Provider 
      value={{ 
        servers,
        fetchServers,
        setServers,
        loading,
        error,
        toggleIpVisibility,
        visibleIpMap
      }}
    >
      {children}
    </GlobalStateContext.Provider>
  );
}

export function useGlobalState() {
  const context = useContext(GlobalStateContext);

  if (context === undefined) {
    throw new Error('useGlobalState must be used within a GlobalStateProvider');
  }

  return context;
}