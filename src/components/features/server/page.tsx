import {invoke} from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { useServerContext } from '@/components/providers/server';
import { usePageContext } from '@/components/providers/page';
import { Button } from '@/components/ui/button';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card';
import { EyeIcon, EyeOffIcon } from 'lucide-react';
import { ConnectButton } from './connect-button';
import { toast } from 'sonner';

export default function ServerPage() {
    const { connectionStatus, connectedServer } = useServerContext();
    const { setCurrentPage } = usePageContext();
    const [error, setError] = useState<string | null>(null);
    const [showIpAddress, setShowIpAddress] = useState(false);

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

            <Card>
                <CardHeader>
                    <CardTitle className="flex items-center justify-between">
                        Server Details
                        <div className="flex items-center">
                            <span className="text-sm mr-2">Status:</span>
                            <span className={`inline-block w-3 h-3 rounded-full mr-1 ${
                                connectionStatus === 'connected' ? 'bg-green-500' : 
                                connectionStatus === 'connecting' ? 'bg-yellow-500' : 'bg-red-500'
                            }`}></span>
                            <span className="text-sm font-normal">
                                {connectionStatus === 'connected' ? 'Connected' : 
                                 connectionStatus === 'connecting' ? 'Connecting...' : 'Disconnected'}
                            </span>
                        </div>
                    </CardTitle>
                </CardHeader>
                <CardContent>
                    <div className="space-y-2">
                        <div className="flex justify-between">
                            <span className="font-medium">Hostname:</span>
                            <span>{connectedServer.hostname}</span>
                        </div>
                        <div className="flex justify-between items-center">
                            <span className="font-medium">IP Address:</span>
                            <div className="flex items-center gap-2">
                                <span>
                                    {connectedServer.ip_address ? (showIpAddress ? connectedServer.ip_address : '••••••••••') : 'N/A'}
                                </span>
                                {connectedServer.ip_address && (
                                    <Button 
                                        variant="ghost" 
                                        size="icon" 
                                        className="h-6 w-6" 
                                        onClick={() => setShowIpAddress(!showIpAddress)}
                                        title={showIpAddress ? "Hide IP Address" : "Show IP Address"}
                                    >
                                        {showIpAddress ? <EyeOffIcon className="h-4 w-4" /> : <EyeIcon className="h-4 w-4" />}
                                    </Button>
                                )}
                                <Button
                                    onClick={async () => {
                                        try {
                                            const result = await invoke('test') as string;

                                            toast.success(result);

                                            console.log(result);
                                        } catch (e) {
                                            console.log(e as string);
                                        }
                                    }}
                                >
                                   Run Test
                                </Button>
                            </div>
                        </div>
                        <div className="flex justify-between">
                            <span className="font-medium">Port:</span>
                            <span>{connectedServer.port}</span>
                        </div>
                        <div className="flex justify-between">
                            <span className="font-medium">Username:</span>
                            <span>{connectedServer.username}</span>
                        </div>
                        {connectedServer.notes && (
                            <div className="mt-4">
                                <span className="font-medium">Notes:</span>
                                <p className="mt-1">{connectedServer.notes}</p>
                            </div>
                        )}
                    </div>
                </CardContent>
            </Card>

            <div className="grid auto-rows-min gap-4 md:grid-cols-3">
                <div className="aspect-video rounded-xl bg-white text-3xl flex justify-center items-center">
                    <p>Server Analytics 1</p>
                </div>
                <div className="aspect-video rounded-xl bg-white text-3xl flex justify-center items-center">
                    <p>Server Analytics 2</p>
                </div>
                <div className="aspect-video rounded-xl bg-white text-3xl flex justify-center items-center">
                    <p>Server Analytics 3</p>
                </div>
            </div>
        </div>
    );
}
