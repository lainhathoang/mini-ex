import "dotenv/config";
import { defineConfig, env } from "prisma/config";

export default defineConfig({
  schema: "crates/database/prisma/portfolio.prisma",
  datasource: {
    url: env("PORTFOLIO_DATABASE_URL"),
  },
});
