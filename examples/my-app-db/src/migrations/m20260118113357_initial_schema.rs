//! Migration: initial_schema
//! Created: 2026-01-18 11:33:57 CET

use dibs::{MigrationContext, Result};

#[dibs::migration("20260118113357-initial_schema")]
pub async fn migrate(ctx: &mut MigrationContext<'_>) -> Result<()> {
    ctx.execute(
        "CREATE TABLE users (
            id BIGINT PRIMARY KEY,
            email TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            bio TEXT,
            created_at TEXT NOT NULL DEFAULT now()
        )",
    )
    .await?;

    ctx.execute(
        "CREATE TABLE posts (
            id BIGINT PRIMARY KEY,
            author_id BIGINT NOT NULL REFERENCES users(id),
            title TEXT NOT NULL,
            body TEXT NOT NULL,
            published BOOLEAN NOT NULL DEFAULT false,
            created_at TEXT NOT NULL DEFAULT now()
        )",
    )
    .await?;

    Ok(())
}
