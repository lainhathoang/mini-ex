-- Market service schema. Mirrors crates/database/prisma/market.prisma.
-- Applied to the market database (MARKET_DATABASE_URL).
-- Idempotent: safe to run against an already-migrated database.

CREATE TABLE IF NOT EXISTS "assets" (
    "name"       TEXT           NOT NULL,
    "symbol"     TEXT           NOT NULL,
    "price"      DECIMAL(30, 10) NOT NULL,
    "created_at" TIMESTAMP(3)   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3)   NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "assets_pkey" PRIMARY KEY ("name")
);
