<script lang="ts">
	import { Chart, Svg, Area, Axis, Tooltip } from "layerchart";
	import { scaleUtc } from "d3-scale";
	import { curveNatural } from "d3-shape";
	import * as Card from "$lib/components/ui/card";
	import type { DailyClick } from "$lib/types";

	let { daily }: { daily: DailyClick[] } = $props();

	let chartData = $derived(
		daily.map((d) => ({ ...d, date: new Date(d.date) }))
	);
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Clicks Over Time</Card.Title>
		<Card.Description>Daily click volume</Card.Description>
	</Card.Header>
	<Card.Content>
		<div class="h-[300px]">
			{#if chartData.length > 0}
				<Chart
					data={chartData}
					x="date"
					xScale={scaleUtc()}
					y="clicks"
					yDomain={[0, null]}
					yNice
					padding={{ top: 20, right: 16, bottom: 36, left: 48 }}
					tooltip={{ mode: "bisect-x" }}
				>
					<Svg>
						<Axis
							placement="bottom"
							format={(d) =>
								d.toLocaleDateString("en", {
									month: "short",
									day: "numeric",
								})}
						/>
						<Axis placement="left" />
						<Area
							fill="var(--chart-1)"
							fillOpacity={0.2}
							curve={curveNatural}
							line={{ stroke: "var(--chart-1)", strokeWidth: 2 }}
						/>
					</Svg>
					<Tooltip.Root let:data>
						<div class="rounded-lg border bg-popover px-3 py-2 text-popover-foreground shadow-md">
							<p class="text-xs text-muted-foreground">
								{data.date.toLocaleDateString("en", { month: "short", day: "numeric", year: "numeric" })}
							</p>
							<p class="text-sm font-semibold" style="color: var(--chart-1)">
								{data.clicks.toLocaleString()} clicks
							</p>
						</div>
					</Tooltip.Root>
				</Chart>
			{:else}
				<div class="flex h-full items-center justify-center text-muted-foreground">
					No click data yet
				</div>
			{/if}
		</div>
	</Card.Content>
</Card.Root>
