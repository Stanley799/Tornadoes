-- 2026-02-20T13:10:00Z
-- Migration: add_date_to_attendance

ALTER TABLE attendance ADD COLUMN date DATE;
