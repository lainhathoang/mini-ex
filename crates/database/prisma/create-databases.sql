-- Creates the application databases. Run on the maintenance (`postgres`)
-- connection, before the per-database schema files.
--
-- CREATE DATABASE cannot run inside a transaction; psql runs each statement in
-- autocommit by default, so the `\gexec` guards below stay idempotent and are
-- safe to re-run against a server where the databases already exist.

SELECT 'CREATE DATABASE "mini-ex-portfolio-db"'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'mini-ex-portfolio-db')\gexec

SELECT 'CREATE DATABASE "mini-ex-market-db"'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'mini-ex-market-db')\gexec
