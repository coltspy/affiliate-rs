<script lang="ts">
	import StatCard from "$lib/components/dashboard/stat-card.svelte";
	import ClicksConversionsChart from "$lib/components/dashboard/clicks-conversions-chart.svelte";
	import RevenueChart from "$lib/components/dashboard/revenue-chart.svelte";
	import ConversionFunnel from "$lib/components/dashboard/conversion-funnel.svelte";
	import ClicksByDayChart from "$lib/components/dashboard/clicks-by-day-chart.svelte";
	import TopAffiliatesTable from "$lib/components/dashboard/top-affiliates-table.svelte";
	import MousePointerClickIcon from "@lucide/svelte/icons/mouse-pointer-click";
	import ArrowRightLeftIcon from "@lucide/svelte/icons/arrow-right-left";
	import DollarSignIcon from "@lucide/svelte/icons/dollar-sign";
	import PercentIcon from "@lucide/svelte/icons/percent";

	let { data } = $props();

	let conversionRate = $derived(
		data.summary.total_clicks > 0
			? (data.summary.total_conversions / data.summary.total_clicks * 100).toFixed(1) + "%"
			: "0%"
	);
</script>

<div class="space-y-4 p-6 pt-8 md:p-8">
	<div>
		<h1 class="text-3xl font-extrabold tracking-tight text-foreground">Analytics</h1>
		<p class="mt-1 text-sm font-semibold text-muted-foreground">Last {data.summary.period_days} days</p>
	</div>

	<div class="grid grid-cols-2 gap-4 lg:grid-cols-4">
		<StatCard title="Total Clicks" value={data.summary.total_clicks.toLocaleString()} icon={MousePointerClickIcon} />
		<StatCard title="Total Conversions" value={data.summary.total_conversions.toLocaleString()} icon={ArrowRightLeftIcon} />
		<StatCard title="Total Revenue" value={"$" + parseFloat(data.summary.total_revenue).toLocaleString()} icon={DollarSignIcon} />
		<StatCard title="Conversion Rate" value={conversionRate} icon={PercentIcon} />
	</div>

	<div class="grid grid-cols-1 gap-4 lg:grid-cols-3">
		<div class="lg:col-span-2">
			<ClicksConversionsChart clicks={data.clicks.daily} conversions={data.conversions.daily} />
		</div>
		<ConversionFunnel summary={data.summary} />
	</div>

	<div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
		<RevenueChart daily={data.conversions.daily} />
		<ClicksByDayChart daily={data.clicks.daily} />
	</div>

	<TopAffiliatesTable affiliates={data.topAffiliates.affiliates} />
</div>
