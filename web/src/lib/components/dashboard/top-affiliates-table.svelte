<script lang="ts">
	import type { TopAffiliate } from "$lib/types";

	let { affiliates }: { affiliates: TopAffiliate[] } = $props();
</script>

<div class="rounded-lg border-2 border-border bg-sidebar p-5">
	<h3 class="text-base font-bold text-foreground">Top Affiliates</h3>
	<p class="text-sm font-medium text-muted-foreground">By performance</p>
	<div class="mt-4 overflow-x-auto">
		{#if affiliates.length > 0}
			<table class="w-full text-sm">
				<thead>
					<tr class="border-b border-border text-xs font-bold text-muted-foreground">
						<th class="px-3 py-2 text-left">Name</th>
						<th class="px-3 py-2 text-left">Code</th>
						<th class="px-3 py-2 text-right">Clicks</th>
						<th class="px-3 py-2 text-right">Conversions</th>
						<th class="px-3 py-2 text-right">Revenue</th>
						<th class="px-3 py-2 text-right">Conv. rate</th>
					</tr>
				</thead>
				<tbody>
					{#each affiliates.slice(0, 10) as affiliate}
						<tr class="border-b border-border/50">
							<td class="px-3 py-3 font-bold text-foreground">{affiliate.name}</td>
							<td class="px-3 py-3 font-mono text-muted-foreground">{affiliate.code}</td>
							<td class="px-3 py-3 text-right font-semibold text-foreground">{affiliate.clicks.toLocaleString()}</td>
							<td class="px-3 py-3 text-right font-semibold text-foreground">{affiliate.conversions.toLocaleString()}</td>
							<td class="px-3 py-3 text-right font-semibold text-foreground">${parseFloat(affiliate.revenue).toLocaleString()}</td>
							<td class="px-3 py-3 text-right font-semibold text-foreground">{affiliate.conversion_rate.toFixed(1)}%</td>
						</tr>
					{/each}
				</tbody>
			</table>
		{:else}
			<div class="flex h-24 items-center justify-center font-semibold text-muted-foreground">No affiliate data yet</div>
		{/if}
	</div>
</div>
