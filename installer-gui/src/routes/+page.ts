import { invoke } from '@tauri-apps/api/core';

interface InstallerCommand {
    subcommands: InstallerSubcommand[];
}

interface InstallerSubcommand {
    label: string;
}

export async function load(): Promise<InstallerCommand> {
    return await invoke('rayhunter_options');
}
