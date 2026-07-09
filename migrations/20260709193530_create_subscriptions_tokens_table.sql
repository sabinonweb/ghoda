-- Add migration script here
-- Create susbcriptions table
CREATE TABLE subscriptions_tokens(
  subscription_token TEXT NOT NULL,
  subscriber_id uuid NOT NULL
    REFERENCES subscriptions(id),
  PRIMARY KEY(subscription_token)
)
