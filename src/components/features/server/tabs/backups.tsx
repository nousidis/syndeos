import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { PlusCircle, Download } from 'lucide-react';

export default function Backups() {
    return (
        <div className="space-y-6 mt-6">
            <div className="flex justify-between items-center">
                <h3 className="text-lg font-medium">Server Backups</h3>
                <div className="flex gap-2">
                    <Button className="flex items-center gap-2" variant="outline">
                        <Download className="h-4 w-4" />
                        Restore Backup
                    </Button>
                    <Button className="flex items-center gap-2">
                        <PlusCircle className="h-4 w-4" />
                        Create Backup
                    </Button>
                </div>
            </div>
            
            <Card>
                <CardHeader>
                    <CardTitle>Backup History</CardTitle>
                </CardHeader>
                <CardContent>
                    <div className="text-center py-6">
                        <p className="text-muted-foreground">No backups found for this server.</p>
                        <p className="text-sm text-muted-foreground mt-1">
                            Click the "Create Backup" button to create a new backup of your server.
                        </p>
                    </div>
                </CardContent>
            </Card>
        </div>
    );
}