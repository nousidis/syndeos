import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { EyeIcon, EyeOffIcon } from 'lucide-react';
import { useState } from 'react';
import { useServerContext } from '@/components/providers/server';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'sonner';

export default function Dashboard() {
    const { connectionStatus, connectedServer } = useServerContext();
    const [showIpAddress, setShowIpAddress] = useState(false);

    return (
        <div className="space-y-6 mt-6">
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
                            <span>{connectedServer?.hostname}</span>
                        </div>
                        <div className="flex justify-between items-center">
                            <span className="font-medium">IP Address:</span>
                            <div className="flex items-center gap-2">
                                <span>
                                    {connectedServer?.ip_address ? (showIpAddress ? connectedServer.ip_address : '••••••••••') : 'N/A'}
                                </span>
                                {connectedServer?.ip_address && (
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
                                            const result = await invoke('install_php', { version: '8.3'}) as string;
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
                            <span>{connectedServer?.port}</span>
                        </div>
                        <div className="flex justify-between">
                            <span className="font-medium">Username:</span>
                            <span>{connectedServer?.username}</span>
                        </div>
                        {connectedServer?.notes && (
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
