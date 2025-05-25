export type Server = {
    id?: number;
    name: string;
    hostname: string;
    ip_address: string;
    port: number;
    username: string;
    ssh_key_id?: number;
    notes?: string;
    settings: string;
    created_at: string;
    updated_at: string;
};

export type Servers = Server[];

export type SshKey = {
    id?: number;
    name: string;
    path: string;
    is_default: boolean;
    created_at: string;
    updated_at: string;
}

export type SshKeys = SshKey[];

export type Setting = {
    id?: number;
    key: string;
    value: string;
};

export type Settings = Setting[];
