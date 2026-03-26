<script lang="ts">
	import { Chart, Svg, Arc } from "layerchart";
	import * as Card from "$lib/components/ui/card";
	import type { TopAffiliate } from "$lib/types";

	let { affiliates }: { affiliates: TopAffiliate[] } = $props();

	let avgRate = $derived(() => {
		if (affiliates.length === 0) return 0;
		let total = affiliates.reduce((sum, a) => sum + a.conversion_rate, 0);
		return Math.round((total / affiliates.length) * 100) / 100;
	});
</script>

<Card.Root class="h-full">
	<Card.Header>
		<Card.Title>Conversion Rate</Card.Title>
		<Card.Description>Average across all affiliates</Card.Description>
	</Card.Header>
	<Card.Content>
		<div class="h-[250px]">
			{#if affiliates.length > 0}
				<Chart>
					<Svg center>
						<Arc
							value={avgRate()}
							domain={[0, Math.max(avgRate() * 2, 10)]}
							range={[-120, 120]}
							innerRadius={-24}
							fill="var(--chart-1)"
							track={{ fill: "var(--muted)", fillOpacity: 0.5 }}
							cornerRadius={4}
						/>
					</Svg>
				</Chart>
				<p class="mt-2 text-center text-3xl font-bold">{avgRate()}%</p>
				<p class="text-center text-sm text-muted-foreground">avg conversion rate</p>
			{:else}
				<div class="flex h-full items-center justify-center text-muted-foreground">
					No conversion data yet
				</div>
			{/if}
		</div>
	</Card.Content>
</Card.Root>
