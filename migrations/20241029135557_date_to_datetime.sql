-- Add migration script here
BEGIN;

ALTER TABLE todo MODIFY date_created DATETIME;
ALTER TABLE todo MODIFY date_due DATETIME;
ALTER TABLE todo MODIFY reminder_date DATETIME;

COMMIT;
