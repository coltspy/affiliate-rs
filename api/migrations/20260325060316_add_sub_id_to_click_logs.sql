ALTER TABLE click_logs ADD COLUMN sub_id TEXT;
CREATE INDEX idx_click_logs_sub_id ON click_logs(sub_id);
