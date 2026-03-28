<script lang="ts">
	import type { Component } from "svelte";

	let { title, value, icon }: { title: string; value: string | number; icon?: Component } = $props();
</script>

<div class="rounded-lg border-2 border-border bg-sidebar p-5">
	<div class="flex items-center justify-between">
		<p class="text-sm font-bold text-foreground">{title}</p>
		{#if icon}
			{@const Icon = icon}
			<Icon class="h-5 w-5 text-muted-foreground" />
		{/if}
	</div>
	<p class="mt-2 text-3xl font-extrabold text-foreground">{value}</p>
</div>
