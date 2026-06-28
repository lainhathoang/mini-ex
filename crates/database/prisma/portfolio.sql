-- Portfolio service schema. Mirrors crates/database/prisma/portfolio.prisma.
-- Applied to the portfolio database (PORTFOLIO_DATABASE_URL).
-- Idempotent: safe to run against an already-migrated database.

-- Enum types. Only the enums referenced by a table are created, matching the
-- schema the services were generated against.
DO $$ BEGIN
    CREATE TYPE "OrderSide" AS ENUM ('BUY', 'SELL');
EXCEPTION WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE "OrderStatus" AS ENUM ('CREATED', 'EXECUTED', 'REJECTED');
EXCEPTION WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE "RejectReason" AS ENUM (
        'INSUFFICIENT_CASH',
        'INSUFFICIENT_ASSET',
        'MARKET_SERVICE_UNAVAILABLE',
        'INVALID_SYMBOL',
        'INVALID_QUANTITY'
    );
EXCEPTION WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS "users" (
    "id"            UUID           NOT NULL,
    "username"      TEXT           NOT NULL,
    "password_hash" TEXT           NOT NULL,
    "cash_balance"  DECIMAL(30, 10) NOT NULL DEFAULT 0,
    "created_at"    TIMESTAMP(3)   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at"    TIMESTAMP(3)   NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX IF NOT EXISTS "users_username_key" ON "users" ("username");

CREATE TABLE IF NOT EXISTS "asset_balances" (
    "user_id"    UUID           NOT NULL,
    "symbol"     TEXT           NOT NULL,
    "quantity"   DECIMAL(30, 10) NOT NULL DEFAULT 0,
    "created_at" TIMESTAMP(3)   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3)   NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "asset_balances_pkey" PRIMARY KEY ("user_id", "symbol")
);

CREATE TABLE IF NOT EXISTS "orders" (
    "id"              UUID           NOT NULL,
    "user_id"         UUID           NOT NULL,
    "client_order_id" TEXT           NOT NULL,
    "symbol"          TEXT           NOT NULL,
    "side"            "OrderSide"    NOT NULL,
    "quantity"        DECIMAL(30, 10) NOT NULL,
    "price"           DECIMAL(30, 10),
    "status"          "OrderStatus"  NOT NULL DEFAULT 'CREATED',
    "reject_reason"   "RejectReason",
    "created_at"      TIMESTAMP(3)   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at"      TIMESTAMP(3)   NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "orders_pkey" PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX IF NOT EXISTS "orders_user_id_client_order_id_key"
    ON "orders" ("user_id", "client_order_id");
