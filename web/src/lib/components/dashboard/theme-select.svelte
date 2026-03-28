<script lang="ts">
	import { untrack } from "svelte";

	let themes = [
		{ value: "teal", label: "Teal" },
		{ value: "pink", label: "Pink" },
		{ value: "blue", label: "Blue" },
		{ value: "orange", label: "Orange" },
		{ value: "zinc", label: "Zinc" },
		{ value: "white", label: "White" },
		{ value: "black", label: "Black" },
	];

	let current = $state("");

	$effect(() => {
		untrack(() => {
			let saved = localStorage.getItem("theme") || "";
			current = saved;
			if (saved) {
				document.documentElement.setAttribute("data-theme", saved);
			}
		});
	});

	function setTheme(theme: string) {
		current = theme;
		document.documentElement.setAttribute("data-theme", theme);
		localStorage.setItem("theme", theme);
	}
</script>

<select
	bind:value={current}
	onchange={(e) => setTheme(e.currentTarget.value)}
	class="w-full appearance-none rounded-lg border border-sidebar-border bg-sidebar-accent px-3 py-1.5 text-xs font-semibold text-sidebar-foreground outline-none transition-colors focus:ring-1 focus:ring-sidebar-ring"
>
	<option value="" disabled>Customize</option>
	{#each themes as theme}
		<option value={theme.value}>{theme.label}</option>
	{/each}
</select>
