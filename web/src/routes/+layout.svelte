<script lang="ts">
	import "../app.css";
	import favicon from "$lib/assets/favicon.svg";
	import type { Snippet } from "svelte";

	let { children }: { children: Snippet } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="min-h-screen bg-background text-foreground">
	{@render children()}
</div>
