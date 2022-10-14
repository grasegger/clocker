-- This file should undo anything in `up.sql`CREATE TABLE hours(
    id INTEGER PRIMARY KEY NOT NULL,
    beginning_with TIMESTAMP NOT NULL,
    hours_per_week REAL NOT NULL
);