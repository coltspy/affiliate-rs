<script lang="ts">
	import { Chart, Svg, Area, Axis, Tooltip } from "layerchart";
	import { scaleUtc } from "d3-scale";
	import { curveNatural } from "d3-shape";
	import type { DailyConversion } from "$lib/types";

	let { daily }: { daily: DailyConversion[] } = $props();

	let chartData = $derived(
		daily.map((d) => ({
			date: new Date(d.date),
			revenue: parseFloat(d.revenue),
		}))
	);
</script>

<div class="rounded-lg border-2 border-border bg-sidebar p-5">
	<h3 class="text-base font-bold text-foreground">Revenue</h3>
	<p class="text-sm font-medium text-muted-foreground">Daily revenue over time</p>
	<div class="mt-4 h-[300px] text-muted-foreground">
		{#if chartData.length > 0}
			<Chart data={chartData} x="date" xScale={scaleUtc()} y="revenue" yDomain={[0, null]} yNice padding={{ top: 20, right: 16, bottom: 36, left: 48 }} tooltip={{ mode: "bisect-x" }}>
				<Svg>
					<Axis placement="bottom" format={(d) => d.toLocaleDateString("en", { month: "short", day: "numeric" })} />
					<Axis placement="left" format={(d) => `$${d}`} />
					<Area fill="var(--chart-2)" fillOpacity={0.15} curve={curveNatural} line={{ stroke: "var(--chart-2)", strokeWidth: 2.5 }} />
				</Svg>
				<Tooltip.Root let:data>
					<div class="rounded-lg border bg-popover px-3 py-2 text-popover-foreground shadow-lg">
						<p class="text-xs font-semibold text-muted-foreground">{data.date.toLocaleDateString("en", { month: "short", day: "numeric", year: "numeric" })}</p>
						<p class="text-sm font-bold" style="color: var(--chart-2)">${data.revenue.toLocaleString()} revenue</p>
					</div>
				</Tooltip.Root>
			</Chart>
		{:else}
			<div class="flex h-full items-center justify-center font-semibold text-muted-foreground">No revenue data yet</div>
		{/if}
	</div>
</div>
