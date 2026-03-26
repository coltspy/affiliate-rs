<script lang="ts">
	import { Chart, Svg, Pie, Tooltip } from "layerchart";
	import * as Card from "$lib/components/ui/card";
	import type { TopAffiliate } from "$lib/types";

	let { affiliates }: { affiliates: TopAffiliate[] } = $props();

	let colors = [
		"var(--chart-1)",
		"var(--chart-2)",
		"var(--chart-3)",
		"var(--chart-4)",
		"var(--chart-5)",
	];

	let chartData = $derived(
		affiliates.slice(0, 5).map((a, i) => ({
			name: a.name,
			clicks: a.clicks,
			fill: colors[i % colors.length],
		}))
	);
</script>

<Card.Root class="h-full">
	<Card.Header>
		<Card.Title>Click Distribution</Card.Title>
		<Card.Description>Top affiliates by clicks</Card.Description>
	</Card.Header>
	<Card.Content>
		<div class="h-[250px]">
			{#if chartData.length > 0}
				<Chart data={chartData} y="clicks">
					<Svg center>
						<Pie innerRadius={0.55} padAngle={0.02} />
					</Svg>
					<Tooltip.Root let:data>
						<div class="rounded-lg border bg-popover px-3 py-2 text-popover-foreground shadow-md">
							<p class="text-xs text-muted-foreground">{data.name}</p>
							<p class="text-sm font-semibold">{data.clicks.toLocaleString()} clicks</p>
						</div>
					</Tooltip.Root>
				</Chart>
			{:else}
				<div class="flex h-full items-center justify-center text-muted-foreground">
					No affiliate data yet
				</div>
			{/if}
		</div>
		<div class="mt-2 flex flex-wrap gap-2 text-xs">
			{#each chartData as item, i}
				<span class="flex items-center gap-1">
					<span
						class="inline-block h-2 w-2 rounded-full"
						style="background: {colors[i % colors.length]}"
					></span>
					{item.name}
				</span>
			{/each}
		</div>
	</Card.Content>
</Card.Root>
