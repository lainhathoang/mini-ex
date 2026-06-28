import "dotenv/config";
import { defineConfig, env } from "prisma/config";

export default defineConfig({
  schema: "crates/database/prisma/market.prisma",
  datasource: {
    url: env("MARKET_DATABASE_URL"),
  },
});
