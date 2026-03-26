CREATE TABLE conversions (
    id             TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    affiliate_id   TEXT NOT NULL REFERENCES affiliates(id),
    event          TEXT NOT NULL,
    amount         NUMERIC(12,2),
    metadata       JSONB,
    sub_id         TEXT,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_conversions_affiliate_id ON conversions(affiliate_id);
CREATE INDEX idx_conversions_event ON conversions(event);
CREATE INDEX idx_conversions_created_at ON conversions(created_at);
CREATE INDEX idx_conversions_sub_id ON conversions(sub_id);
