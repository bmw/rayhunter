<script lang="ts">
    import type { InstallerSubcommand } from '$lib/types.ts';

    let {
        setDevice,
        subcommands,
    }: {
        setDevice: (s: InstallerSubcommand) => void;
        subcommands: InstallerSubcommand[];
    } = $props();

    let selection = $state<InstallerSubcommand | null>(null);
    let buttonEnabled = $derived(selection !== null);

    function on_click() {
        if (selection !== null) {
            setDevice(selection);
        }
    }
</script>

<div class="flex flex-col gap-4 items-center pt-4 text-xl">
    <img alt="rayhunter orca logo" class="h-35 w-35" src="/orca.svg" />
    <h1 class="font-bold text-3xl">Install Rayhunter</h1>
    <label class="text-gray-600" for="device-select">
        Select your device and installation method
    </label>
    <div class="flex gap-4">
        <select
            bind:value={selection}
            class="border border-gray-300 text-base"
            name="devices"
            id="device-select"
        >
            <option value={null}></option>
            {#each subcommands as subcommand (subcommand.label)}
                <option value={subcommand}>{subcommand.label}</option>
            {/each}
        </select>
        <button
            class="{buttonEnabled ? 'cursor-pointer' : ''}
                bg-rayhunter-blue px-6 py-2 rounded-lg shadow-md text-white"
            disabled={!buttonEnabled}
            onclick={on_click}
        >
            Next
        </button>
    </div>
</div>
