DROP TABLE IF EXISTS "ChoiceTrades";

CREATE TABLE IF NOT EXISTS "ChoiceTrades" (
  "id" VARCHAR(64) PRIMARY KEY,
  "transaction_hash" VARCHAR(128),
  "contract_address" VARCHAR(128),
  "action" VARCHAR(64),
  "ask_asset" VARCHAR(128),
  "burn_amount" VARCHAR(128),
  "commission_amount" VARCHAR(128),
  "fee_wallet_amount" VARCHAR(128),
  "offer_amount" VARCHAR(128),
  "offer_asset" VARCHAR(128),
  "pool_amount" VARCHAR(128),
  "receiver" VARCHAR(128),
  "return_amount" VARCHAR(128),
  "sender" VARCHAR(128),
  "spread_amount" VARCHAR(128),
  "msg_index" VARCHAR(64),
  "block_number" BIGINT,
  "block_time" TIMESTAMP
);
