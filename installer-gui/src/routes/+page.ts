import { invoke } from '@tauri-apps/api/core';

import type { InstallerCommand } from '$lib/types.ts';

export function load(): Promise<InstallerCommand> {
    return invoke('rayhunter_options');
}
