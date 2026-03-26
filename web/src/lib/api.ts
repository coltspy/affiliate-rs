import type {
	DashboardSummary,
	DailyClicksResponse,
	DailyConversionsResponse,
	TopAffiliatesResponse
} from "./types";

const BASE_URL = import.meta.env.VITE_API_URL || "http://localhost:3000";
const API_KEY = import.meta.env.VITE_API_KEY || "";

async function get<T>(path: string, params?: Record<string, string>): Promise<T> {
	const url = new URL(path, BASE_URL);
	if (params) {
		for (const [key, value] of Object.entries(params)) {
			url.searchParams.set(key, value);
		}
	}
	const res = await fetch(url.toString(), {
		headers: { Authorization: `Bearer ${API_KEY}` }
	});
	if (!res.ok) throw new Error(`API error: ${res.status}`);
	return res.json();
}

export function fetchSummary(days = 30) {
	return get<DashboardSummary>("/api/v1/dashboard/summary", { days: String(days) });
}

export function fetchDailyClicks(days = 30) {
	return get<DailyClicksResponse>("/api/v1/dashboard/clicks", { days: String(days) });
}

export function fetchDailyConversions(days = 30) {
	return get<DailyConversionsResponse>("/api/v1/dashboard/conversions", { days: String(days) });
}

export function fetchTopAffiliates(days = 30, limit = 10) {
	return get<TopAffiliatesResponse>("/api/v1/dashboard/top-affiliates", {
		days: String(days),
		limit: String(limit)
	});
}
