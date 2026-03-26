<script lang="ts">
	import StatCard from "$lib/components/dashboard/stat-card.svelte";
	import ClicksAreaChart from "$lib/components/dashboard/clicks-area-chart.svelte";
	import ConversionsLineChart from "$lib/components/dashboard/conversions-line-chart.svelte";
	import AffiliatesPieChart from "$lib/components/dashboard/affiliates-pie-chart.svelte";
	import AffiliatesRadarChart from "$lib/components/dashboard/affiliates-radar-chart.svelte";
	import ConversionRadialChart from "$lib/components/dashboard/conversion-radial-chart.svelte";
	import ThemeSelect from "$lib/components/dashboard/theme-select.svelte";

	let { data } = $props();
</script>

<div class="mx-auto max-w-7xl space-y-6 p-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold tracking-tight">Analytics</h1>
			<p class="text-muted-foreground">Last {data.summary.period_days} days</p>
		</div>
		<ThemeSelect />
	</div>

	<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
		<StatCard title="Total Clicks" value={data.summary.total_clicks.toLocaleString()} />
		<StatCard title="Total Conversions" value={data.summary.total_conversions.toLocaleString()} />
		<StatCard
			title="Total Revenue"
			value={"$" + parseFloat(data.summary.total_revenue).toLocaleString()}
		/>
		<StatCard title="Active Affiliates" value={data.summary.active_affiliates} />
	</div>

	<ClicksAreaChart daily={data.clicks.daily} />

	<ConversionsLineChart daily={data.conversions.daily} />

	<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
		<AffiliatesPieChart affiliates={data.topAffiliates.affiliates} />
		<AffiliatesRadarChart affiliates={data.topAffiliates.affiliates} />
		<ConversionRadialChart affiliates={data.topAffiliates.affiliates} />
	</div>
</div>
