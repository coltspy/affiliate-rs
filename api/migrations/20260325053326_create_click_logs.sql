CREATE TABLE click_logs (
    id           TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    affiliate_id TEXT NOT NULL REFERENCES affiliates(id),
    ip           TEXT,
    user_agent   TEXT,
    referer      TEXT,
    clicked_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_click_logs_affiliate_id ON click_logs(affiliate_id);
CREATE INDEX idx_click_logs_clicked_at ON click_logs(clicked_at);
