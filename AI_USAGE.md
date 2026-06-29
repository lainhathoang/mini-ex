# AI Usage

## Tools Used

| Tool                       | Role                                                                                                                                                                                                |
| -------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Claude Code**            | Primary driver. Used for codebase scouting, brainstorming, planning, implementation, code review, and conventional commits.                                                                         |
| **Subagents / skills**     | Used `brainstorm` + `plan` for design and phased planning, `code-reviewer` for review before commits, `git-manager` for conventional commits, and `journal-writer` / `planner` for context capture. |
| **ChatGPT / Claude (web)** | Secondary support. Used for questions, API/syntax sanity checks, and explanations outside the editor. Nothing was committed directly from these tools.                                              |

All AI output was reviewed by a human before being committed.

---

## Key Prompts

Representative prompts, paraphrased, that produced committed work:

- “Both services share one `database` crate and one `DATABASE_URL`; give each service its own Postgres DB. Scout first, propose approaches, then plan.”

- “Add order execution buy/sell handlers for the portfolio service, following the logic I provided.”
  - Output:
    - `crates/portfolio-service/src/handlers/orders/execution/{buy,sell,mod}.rs`

- “Add Docker setup for local development, including Docker Compose, service Dockerfiles, migration Dockerfile, and environment variable configuration.”
  - Output:
    - `docker-compose.yml`
    - `Dockerfile`
    - `Dockerfile.migrate`

- “Add Swagger/OpenAPI docs for the portfolio service, market service.”
  - Output:
    - `crates/portfolio-service/docs/{openapi.yml,swagger.html,scalar.html}`

- “Write idempotency tests for orders using `client_order_id`.”
  - Output:
    - `crates/database/tests/orders_idempotency.rs`

* “Build a simple static front-end for testing the full user flow using `crates/portfolio-service/docs`, `crates/market-service/docs`, following `DESIGN.md`.”
  - Output:
    - `_mini-ex-front-end/index.html`
    - `_mini-ex-front-end/DESIGN.md`

* “Implement cron job for Market Service to fetch crypto prices from CoinGecko and persist them into DB.”
  - References:
    - `https://docs.coingecko.com/`
    - `https://api.coingecko.com/api/v3/coins/markets`
    - `https://docs.rs/tokio/latest/tokio/time/fn.interval.html`
    - `https://docs.rs/reqwest/latest/reqwest/`

  - Requirements:
    - Fetch top 100 crypto assets from CoinGecko.
    - Store only required fields: `symbol`, `name`, and USD price.
    - Run automatically on a fixed interval.
    - Use upsert logic to insert new assets or update existing prices.
    - Handle API/network/database errors without crashing the service.
    - Add logging for successful and failed sync attempts.
    - Keep the implementation modular: separate API fetching, DB syncing, and cron startup logic.

  - Output:
    - `market-service cron job implementation`
    - `CoinGecko fetch logic`
    - `database upsert logic`
    - `environment variable configuration`

---

## Tasks Delegated to AI

| Task                                 | Output                                                                                                                                          |
| ------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| Per-service DB wiring                | `shared/src/env.rs` enum split, `state.rs` in both services, and Prisma config split.                                                           |
| Market price cron job                | Background job for fetching crypto prices from CoinGecko, syncing `symbol`, `name`, and USD price into the Market Service DB with upsert logic. |
| Order execution logic implementation | Buy/sell order execution handlers for the portfolio service, following the logic I provided.                                                    |
| Schema and migrations                | Prisma schemas, raw SQL migrations, and serde-mapped DB enums.                                                                                  |
| Integration tests                    | Transaction-rolled-back idempotency tests, skipped when no DB is reachable.                                                                     |
| Infra and docs                       | Docker setup and OpenAPI/Swagger docs.                                                                                                          |
| Reviews and commits                  | `code-reviewer` passes and conventional commit messages.                                                                                        |

---

## Accepted vs. Modified

### Accepted largely as generated

- Conventional commit messages.
- Docker and OpenAPI scaffolding.
- Mechanical refactors and formatting.
- Basic CoinGecko fetch logic and cron-style background sync structure.

### Modified before keeping

- Order execution and schema changes were reviewed and manually adjusted to match existing repository conventions, including entity/repository boundaries and enum naming, instead of being committed verbatim.
- Database migration flow was reviewed and adjusted before being kept.
- The market price cron job was reviewed and adjusted before keeping, especially around error handling, logging, environment configuration, and separating API fetch logic from database sync logic.
- CoinGecko response handling was reduced to only the required fields: `symbol`, `name`, and USD price, instead of keeping unnecessary response data.

---

## Incorrect AI Output and How It Was Handled

### 1. Half-migrated, broken database wiring

During the DB-per-service change, the AI updated `.env` to use:

- `MARKET_DATABASE_URL`
- `PORTFOLIO_DATABASE_URL`

However, some service code still referenced the removed `DATABASE_URL`, causing both services to fail at startup.

I caught this during human review in the scout/brainstorm phase. I handled it by adding an env-resolution test, updating `shared/src/env.rs`, fixing both service `state.rs` files, and adding the acceptance criterion:

> No remaining references to `DATABASE_URL` or `Env::DatabaseUrl`.

---

### 2. Overly large order creation file

For the order execution feature, the AI initially put the create-order flow, buy logic, sell logic, and helper functions into one large file. The code worked conceptually, but it was hard to review and did not match the project structure.

I handled this by defining a clearer module structure, then asking the AI to refactor the code into focused files:

- `create.rs`
- `get.rs`
- `execution/buy.rs`
- `execution/sell.rs`
- `execution/mod.rs`
- `mod.rs`

This made the implementation easier to review, test, and maintain.
