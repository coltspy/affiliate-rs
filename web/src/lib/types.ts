export interface DashboardSummary {
	total_clicks: number;
	total_conversions: number;
	total_revenue: string;
	active_affiliates: number;
	period_days: number;
}

export interface DailyClick {
	date: string;
	clicks: number;
}

export interface DailyClicksResponse {
	period_days: number;
	daily: DailyClick[];
}

export interface DailyConversion {
	date: string;
	conversions: number;
	revenue: string;
}

export interface DailyConversionsResponse {
	period_days: number;
	daily: DailyConversion[];
}

export interface TopAffiliate {
	code: string;
	name: string;
	clicks: number;
	conversions: number;
	revenue: string;
	conversion_rate: number;
}

export interface TopAffiliatesResponse {
	period_days: number;
	affiliates: TopAffiliate[];
}
