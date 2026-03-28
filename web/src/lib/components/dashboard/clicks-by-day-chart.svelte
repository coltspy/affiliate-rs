<script lang="ts">
	import { Chart, Svg, Bars, Axis, Tooltip } from "layerchart";
	import { scaleBand } from "d3-scale";
	import type { DailyClick } from "$lib/types";

	let { daily }: { daily: DailyClick[] } = $props();

	let dayNames = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

	let chartData = $derived.by(() => {
		let totals = [0, 0, 0, 0, 0, 0, 0];
		let counts = [0, 0, 0, 0, 0, 0, 0];
		for (let d of daily) {
			let day = new Date(d.date).getDay();
			totals[day] += d.clicks;
			counts[day]++;
		}
		return dayNames.map((name, i) => ({
			day: name,
			clicks: counts[i] > 0 ? Math.round(totals[i] / counts[i]) : 0,
		}));
	});
</script>

<div class="rounded-lg border-2 border-border bg-sidebar p-5">
	<h3 class="text-base font-bold text-foreground">Clicks by day</h3>
	<p class="text-sm font-medium text-muted-foreground">Average clicks per day of week</p>
	<div class="mt-4 h-[250px] text-muted-foreground">
		{#if daily.length > 0}
			<Chart data={chartData} x="day" xScale={scaleBand().padding(0.3)} y="clicks" yDomain={[0, null]} yNice padding={{ top: 20, right: 16, bottom: 36, left: 48 }}>
				<Svg>
					<Axis placement="bottom" />
					<Axis placement="left" />
					<Bars fill="var(--chart-1)" radius={4} />
				</Svg>
				<Tooltip.Root let:data>
					<div class="rounded-lg border bg-popover px-3 py-2 text-popover-foreground shadow-lg">
						<p class="text-xs font-semibold text-muted-foreground">{data.day}</p>
						<p class="text-sm font-bold" style="color: var(--chart-1)">{data.clicks.toLocaleString()} avg clicks</p>
					</div>
				</Tooltip.Root>
			</Chart>
		{:else}
			<div class="flex h-full items-center justify-center font-semibold text-muted-foreground">No click data yet</div>
		{/if}
	</div>
</div>
