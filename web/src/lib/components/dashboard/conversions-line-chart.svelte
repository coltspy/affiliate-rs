<script lang="ts">
	import { Chart, Svg, Spline, Axis, Tooltip } from "layerchart";
	import { scaleUtc } from "d3-scale";
	import { curveNatural } from "d3-shape";
	import * as Card from "$lib/components/ui/card";
	import type { DailyConversion } from "$lib/types";

	let { daily }: { daily: DailyConversion[] } = $props();

	let chartData = $derived(
		daily.map((d) => ({
			date: new Date(d.date),
			conversions: d.conversions,
			revenue: parseFloat(d.revenue),
		}))
	);
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Conversions & Revenue</Card.Title>
		<Card.Description>Daily conversions and revenue over time</Card.Description>
	</Card.Header>
	<Card.Content>
		<div class="h-[300px]">
			{#if chartData.length > 0}
				<Chart
					data={chartData}
					x="date"
					xScale={scaleUtc()}
					y="conversions"
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
						<Spline
							stroke="var(--chart-1)"
							strokeWidth={2}
							curve={curveNatural}
						/>
						<Spline
							y="revenue"
							stroke="var(--chart-2)"
							strokeWidth={2}
							curve={curveNatural}
						/>
					</Svg>
					<Tooltip.Root let:data>
						<div class="rounded-lg border bg-popover px-3 py-2 text-popover-foreground shadow-md">
							<p class="text-xs text-muted-foreground">
								{data.date.toLocaleDateString("en", { month: "short", day: "numeric", year: "numeric" })}
							</p>
							<p class="text-sm font-semibold" style="color: var(--chart-1)">
								{data.conversions.toLocaleString()} conversions
							</p>
							<p class="text-sm font-semibold" style="color: var(--chart-2)">
								${data.revenue.toLocaleString()} revenue
							</p>
						</div>
					</Tooltip.Root>
				</Chart>
			{:else}
				<div class="flex h-full items-center justify-center text-muted-foreground">
					No conversion data yet
				</div>
			{/if}
		</div>
		<div class="mt-3 flex items-center gap-4 text-sm">
			<span class="flex items-center gap-1.5">
				<span class="inline-block h-2.5 w-2.5 rounded-full" style="background: var(--chart-1)"></span>
				Conversions
			</span>
			<span class="flex items-center gap-1.5">
				<span class="inline-block h-2.5 w-2.5 rounded-full" style="background: var(--chart-2)"></span>
				Revenue
			</span>
		</div>
	</Card.Content>
</Card.Root>
