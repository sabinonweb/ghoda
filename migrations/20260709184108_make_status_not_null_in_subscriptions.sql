-- Add migration script here
-- We wrap the whole migration in a transaction to make sure
-- it succeeds or fails immediately
BEGIN;
  -- Backfill 'status' for historical enteries
  UPDATE subscriptions
    SET status = 'confirmed'
    WHERE status IS NULL;
  
  ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;
