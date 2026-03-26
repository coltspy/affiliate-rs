<script lang="ts">
	import { Chart, Svg, Area, Spline, Axis } from "layerchart";
	import { scaleBand } from "d3-scale";
	import * as Card from "$lib/components/ui/card";
	import type { TopAffiliate } from "$lib/types";

	let { affiliates }: { affiliates: TopAffiliate[] } = $props();

	let colors = [
		"var(--chart-1)",
		"var(--chart-2)",
		"var(--chart-3)",
	];

	let top3 = $derived(affiliates.slice(0, 3));

	let metrics = ["Clicks", "Conversions", "Revenue", "Conv. Rate"];

	function normalize(values: number[]): number[] {
		let max = Math.max(...values, 1);
		return values.map((v) => (v / max) * 100);
	}

	let normalizedData = $derived(() => {
		if (top3.length === 0) return [];
		let clickVals = normalize(top3.map((a) => a.clicks));
		let convVals = normalize(top3.map((a) => a.conversions));
		let revVals = normalize(top3.map((a) => parseFloat(a.revenue)));
		let rateVals = normalize(top3.map((a) => a.conversion_rate));
		return metrics.map((metric, mi) => {
			let row: Record<string, string | number> = { metric };
			top3.forEach((a, ai) => {
				let vals = [clickVals, convVals, revVals, rateVals];
				row[a.code] = vals[mi][ai];
			});
			return row;
		});
	});
</script>

<Card.Root class="h-full">
	<Card.Header>
		<Card.Title>Affiliate Comparison</Card.Title>
		<Card.Description>Top 3 normalized performance</Card.Description>
	</Card.Header>
	<Card.Content>
		<div class="h-[250px]">
			{#if top3.length > 0}
				<Chart
					data={normalizedData()}
					x="metric"
					xScale={scaleBand()}
					y={top3[0]?.code}
					yDomain={[0, 100]}
					radial
					padding={{ top: 20, right: 20, bottom: 20, left: 20 }}
				>
					<Svg center>
						<Axis placement="angle" />
						{#each top3 as affiliate, i}
							<Area
								y={affiliate.code}
								fill={colors[i]}
								fillOpacity={0.15}
								line={{ stroke: colors[i], strokeWidth: 2 }}
							/>
						{/each}
					</Svg>
				</Chart>
			{:else}
				<div class="flex h-full items-center justify-center text-muted-foreground">
					No affiliate data yet
				</div>
			{/if}
		</div>
		<div class="mt-2 flex flex-wrap gap-2 text-xs">
			{#each top3 as affiliate, i}
				<span class="flex items-center gap-1">
					<span
						class="inline-block h-2 w-2 rounded-full"
						style="background: {colors[i]}"
					></span>
					{affiliate.name}
				</span>
			{/each}
		</div>
	</Card.Content>
</Card.Root>
