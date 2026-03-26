<script lang="ts">
	let themes = [
		{ value: "pink", label: "Pink" },
		{ value: "blue", label: "Blue" },
		{ value: "green", label: "Green" },
		{ value: "orange", label: "Orange" },
		{ value: "zinc", label: "Zinc" },
	];

	let current = $state("pink");

	$effect(() => {
		if (typeof window !== "undefined") {
			current = localStorage.getItem("theme") || "pink";
			document.documentElement.setAttribute("data-theme", current);
		}
	});

	function setTheme(theme: string) {
		current = theme;
		document.documentElement.setAttribute("data-theme", theme);
		localStorage.setItem("theme", theme);
	}
</script>

<div class="relative">
	<select
		value={current}
		onchange={(e) => setTheme(e.currentTarget.value)}
		class="appearance-none rounded-md border border-border bg-background px-3 py-1.5 pr-8 text-sm text-foreground shadow-sm outline-none focus:ring-2 focus:ring-ring"
	>
		{#each themes as theme}
			<option value={theme.value}>{theme.label}</option>
		{/each}
	</select>
	<svg
		class="pointer-events-none absolute right-2 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground"
		xmlns="http://www.w3.org/2000/svg"
		viewBox="0 0 20 20"
		fill="currentColor"
	>
		<path
			fill-rule="evenodd"
			d="M5.22 8.22a.75.75 0 0 1 1.06 0L10 11.94l3.72-3.72a.75.75 0 1 1 1.06 1.06l-4.25 4.25a.75.75 0 0 1-1.06 0L5.22 9.28a.75.75 0 0 1 0-1.06Z"
			clip-rule="evenodd"
		/>
	</svg>
</div>
