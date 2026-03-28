<script lang="ts">
	import type { DashboardSummary } from "$lib/types";

	let { summary }: { summary: DashboardSummary } = $props();

	let steps = $derived([
		{ label: "Clicks", value: summary.total_clicks },
		{ label: "Conversions", value: summary.total_conversions },
		{ label: "Revenue", value: parseFloat(summary.total_revenue) },
	]);

	let maxValue = $derived(Math.max(steps[0].value, 1));
</script>

<div class="flex h-full flex-col rounded-lg border-2 border-border bg-sidebar p-5">
	<h3 class="text-base font-bold text-foreground">Conversion Funnel</h3>
	<p class="text-sm font-medium text-muted-foreground">Click to revenue pipeline</p>
	<div class="mt-4 flex flex-1 flex-col justify-center gap-4">
		{#each steps as step, i}
			{@const width = i === 2 ? (summary.total_conversions > 0 ? Math.max((step.value / (parseFloat(summary.total_revenue) || 1)) * 100, 15) : 0) : Math.max((step.value / maxValue) * 100, 15)}
			{@const barWidth = i === 0 ? 100 : i === 1 ? Math.max((step.value / maxValue) * 100, 8) : Math.max((summary.total_conversions / maxValue) * 100 * 0.7, 6)}
			<div>
				<div class="mb-1 flex items-baseline justify-between">
					<span class="text-sm font-bold text-foreground">{step.label}</span>
					<span class="text-sm font-extrabold text-foreground">
						{i === 2 ? "$" + step.value.toLocaleString() : step.value.toLocaleString()}
					</span>
				</div>
				<div class="h-3 w-full overflow-hidden rounded-full bg-border/50">
					<div
						class="h-full rounded-full transition-all duration-500"
						style="width: {barWidth}%; background: var(--chart-{i + 1})"
					></div>
				</div>
				{#if i === 0 && summary.total_clicks > 0}
					<p class="mt-0.5 text-xs text-muted-foreground">
						{(summary.total_conversions / summary.total_clicks * 100).toFixed(1)}% convert
					</p>
				{/if}
			</div>
		{/each}
	</div>
</div>
