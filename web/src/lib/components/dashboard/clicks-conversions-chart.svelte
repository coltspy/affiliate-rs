<script lang="ts">
	import { Chart, Svg, Area, Axis, Tooltip } from "layerchart";
	import { scaleUtc } from "d3-scale";
	import { curveNatural } from "d3-shape";
	import type { DailyClick, DailyConversion } from "$lib/types";

	let { clicks, conversions }: { clicks: DailyClick[]; conversions: DailyConversion[] } = $props();

	let chartData = $derived.by(() => {
		let map = new Map<string, { date: Date; clicks: number; conversions: number }>();
		for (let c of clicks) {
			map.set(c.date, { date: new Date(c.date), clicks: c.clicks, conversions: 0 });
		}
		for (let c of conversions) {
			let existing = map.get(c.date);
			if (existing) existing.conversions = c.conversions;
			else map.set(c.date, { date: new Date(c.date), clicks: 0, conversions: c.conversions });
		}
		return Array.from(map.values()).sort((a, b) => a.date.getTime() - b.date.getTime());
	});

	let yMax = $derived(Math.max(...chartData.map((d) => Math.max(d.clicks, d.conversions)), 1));
</script>

<div class="rounded-lg border-2 border-border bg-sidebar p-5">
	<h3 class="text-base font-bold text-foreground">Clicks & Conversions</h3>
	<p class="text-sm font-medium text-muted-foreground">Daily traffic and conversion trend</p>
	<div class="mt-4 h-[300px] text-muted-foreground">
		{#if chartData.length > 0}
			<Chart data={chartData} x="date" xScale={scaleUtc()} y="clicks" yDomain={[0, yMax]} yNice padding={{ top: 20, right: 16, bottom: 36, left: 48 }} tooltip={{ mode: "bisect-x" }}>
				<Svg>
					<Axis placement="bottom" format={(d) => d.toLocaleDateString("en", { month: "short", day: "numeric" })} />
					<Axis placement="left" />
					<Area fill="var(--chart-1)" fillOpacity={0.15} curve={curveNatural} line={{ stroke: "var(--chart-1)", strokeWidth: 2.5 }} />
					<Area y="conversions" fill="var(--chart-2)" fillOpacity={0.15} curve={curveNatural} line={{ stroke: "var(--chart-2)", strokeWidth: 2.5 }} />
				</Svg>
				<Tooltip.Root let:data>
					<div class="rounded-lg border bg-popover px-3 py-2 text-popover-foreground shadow-lg">
						<p class="text-xs font-semibold text-muted-foreground">{data.date.toLocaleDateString("en", { month: "short", day: "numeric", year: "numeric" })}</p>
						<p class="text-sm font-bold" style="color: var(--chart-1)">{data.clicks.toLocaleString()} clicks</p>
						<p class="text-sm font-bold" style="color: var(--chart-2)">{data.conversions.toLocaleString()} conversions</p>
					</div>
				</Tooltip.Root>
			</Chart>
		{:else}
			<div class="flex h-full items-center justify-center font-semibold text-muted-foreground">No data yet</div>
		{/if}
	</div>
	<div class="mt-3 flex gap-4 text-xs font-semibold">
		<span class="flex items-center gap-1.5">
			<span class="inline-block h-2.5 w-2.5 rounded-full" style="background: var(--chart-1)"></span>
			Clicks
		</span>
		<span class="flex items-center gap-1.5">
			<span class="inline-block h-2.5 w-2.5 rounded-full" style="background: var(--chart-2)"></span>
			Conversions
		</span>
	</div>
</div>
