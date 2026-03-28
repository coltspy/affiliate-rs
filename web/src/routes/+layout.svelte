<script lang="ts">
	import "../app.css";
	import favicon from "$lib/assets/favicon.svg";
	import type { Snippet } from "svelte";
	import AppSidebar from "$lib/components/app-sidebar.svelte";
	import MenuIcon from "@lucide/svelte/icons/menu";

	let { children }: { children: Snippet } = $props();
	let sidebarOpen = $state(false);
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="min-h-screen bg-sidebar font-sans text-foreground">
	<AppSidebar bind:open={sidebarOpen} />
	<main class="min-h-screen bg-background md:ml-60">
		<div class="flex h-14 items-center px-6 md:hidden">
			<button onclick={() => (sidebarOpen = true)} class="rounded-lg p-1 text-foreground">
				<MenuIcon class="h-5 w-5" />
			</button>
		</div>
		{@render children()}
	</main>
</div>
