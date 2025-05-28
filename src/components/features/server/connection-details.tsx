import React from 'react';
import { useServerContext, ConnectionStatuses } from '@/components/providers/server';
import { usePageContext } from '@/components/providers/page';
import { Button } from '@/components/ui/button';
import { IconServer } from '@tabler/icons-react';

export function ConnectionDetails() {
  const { connectionStatus, connectedServer } = useServerContext();
  const { currentPage, setCurrentPage } = usePageContext();
  
  const shouldShow =
    connectionStatus === ConnectionStatuses.connected &&
    connectedServer &&
    currentPage !== 'server';
  
  if (!shouldShow) {
    return null;
  }

  const handleShowServerPage = () => setCurrentPage('server');
  
  return (
    <div className="fixed bottom-4 right-4 z-50">
      <Button
        variant="outline"
        className="border-green-500 shadow-md flex items-center gap-2 hover:bg-accent/25"
        onClick={handleShowServerPage}
      >
        <div className="flex items-center gap-2">
          <span className="inline-block w-2 h-2 rounded-full bg-green-500"></span>
          <IconServer className="h-4 w-4" />
          <span>Connected to {connectedServer.name}</span>
        </div>
      </Button>
    </div>
  );
}