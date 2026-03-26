import { fetchSummary, fetchDailyClicks, fetchDailyConversions, fetchTopAffiliates } from "$lib/api";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
	const [summary, clicks, conversions, topAffiliates] = await Promise.all([
		fetchSummary(),
		fetchDailyClicks(),
		fetchDailyConversions(),
		fetchTopAffiliates()
	]);

	return { summary, clicks, conversions, topAffiliates };
};
