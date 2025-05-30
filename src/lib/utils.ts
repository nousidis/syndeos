import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"
import { useSettings } from "@/components/providers/settings"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function fdate(timestamp: string): string {
  const { settings } = useSettings();

  const date = new Date(timestamp);

  // Format options
  const options: Intl.DateTimeFormatOptions = {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: true
  };

  switch (settings['ui/date_format']) {
    case "MM/DD/YYYY":
      return date.toLocaleString('en-US', options); // ex. April 21, 2025 at 11:53:53 PM
      break;
    case "DD/MM/YYYY":
      return date.toLocaleString('en-GB', options); // ex. 21 April 2025 at 11:53:53 pm
      break;
    case "YYYY-MM-DD":
      return date.toISOString(); // ex. 2025-04-22T03:53:53.941Z
      break;
    default:
      return date.toLocaleString('en-US', options);
      break;
  }
}

