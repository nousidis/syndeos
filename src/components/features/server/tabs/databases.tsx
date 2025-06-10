import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { PlusCircle } from 'lucide-react';

export default function Databases() {
    return (
        <div className="space-y-6 mt-6">
            <div className="flex justify-between items-center">
                <h3 className="text-lg font-medium">Manage Databases</h3>
                <Button className="flex items-center gap-2">
                    <PlusCircle className="h-4 w-4" />
                    Add Database
                </Button>
            </div>
            
            <Card>
                <CardHeader>
                    <CardTitle>Your Databases</CardTitle>
                </CardHeader>
                <CardContent>
                    <div className="text-center py-6">
                        <p className="text-muted-foreground">No databases found on this server.</p>
                        <p className="text-sm text-muted-foreground mt-1">
                            Click the "Add Database" button to create a new database.
                        </p>
                    </div>
                </CardContent>
            </Card>
        </div>
    );
}