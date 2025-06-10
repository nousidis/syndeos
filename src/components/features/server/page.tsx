import { useEffect, useState } from 'react';
import { useServerContext } from '@/components/providers/server';
import { usePageContext } from '@/components/providers/page';
import { Button } from '@/components/ui/button';
import { ConnectButton } from './connect-button';
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

// Import tab components
import Dashboard from './tabs/dashboard';
import Websites from './tabs/websites';
import Databases from './tabs/databases';
import Backups from './tabs/backups';

export default function ServerPage() {
    const { connectionStatus, connectedServer } = useServerContext();
    const { setCurrentPage } = usePageContext();
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        if (!connectedServer) {
            setCurrentPage('servers');
        }
    }, [connectedServer?.id]);

    const handleBackToServers = () => {
        setCurrentPage('servers');
    };

    if (!connectedServer) {
        return (
            <div className="flex flex-col justify-center items-center h-full gap-4">
                <p>No server selected or server data not available.</p>
                <Button onClick={handleBackToServers}>Back to Servers</Button>
            </div>
        );
    }

    return (
        <div className="space-y-6">
            <div className="flex justify-between items-center">
                <h2 className="text-2xl font-bold">{connectedServer.name}</h2>
                <div className="flex gap-2">
                    <ConnectButton 
                        server={connectedServer} 
                        variant="default"
                    />
                    <Button onClick={handleBackToServers} variant="outline" className="hover:bg-accent/25">
                        Back to Servers
                    </Button>
                </div>
            </div>

            {error && (
                <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative mb-4">
                    <strong className="font-bold">Error: </strong>
                    <span className="block sm:inline">{error}</span>
                </div>
            )}

            <Tabs defaultValue="dashboard" className="w-full">
                <TabsList className="grid grid-cols-4 mb-4">
                    <TabsTrigger value="dashboard">Dashboard</TabsTrigger>
                    <TabsTrigger value="websites">Websites</TabsTrigger>
                    <TabsTrigger value="databases">Databases</TabsTrigger>
                    <TabsTrigger value="backups">Backups</TabsTrigger>
                </TabsList>
                <TabsContent value="dashboard">
                    <Dashboard />
                </TabsContent>
                <TabsContent value="websites">
                    <Websites />
                </TabsContent>
                <TabsContent value="databases">
                    <Databases />
                </TabsContent>
                <TabsContent value="backups">
                    <Backups />
                </TabsContent>
            </Tabs>
        </div>
    );
}
