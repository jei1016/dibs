//! Database service binary for my-app.
//!
//! Usage:
//!   my-app-db          - Run the dibs service (connects back to CLI via roam)
//!   my-app-db seed     - Seed the database with sample data

use my_app_db::{Post, User};
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Touch the types so they're not dead code eliminated
    let _ = (std::any::type_name::<User>(), std::any::type_name::<Post>());

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "seed" {
        seed().await?;
    } else {
        // Run the dibs service (connects back to CLI via roam)
        dibs::run_service();
    }

    Ok(())
}

async fn seed() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/dibs_test".to_string());

    println!("Seeding database: {}", database_url);

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    // Spawn connection handler
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Clear existing data (in reverse FK order)
    client.execute("DELETE FROM posts", &[]).await?;
    client.execute("DELETE FROM users", &[]).await?;
    println!("Cleared existing data");

    // Seed users
    let users = [
        (
            1i64,
            "alice@example.com",
            "Alice",
            Some("Software engineer who loves Rust"),
        ),
        (2i64, "bob@example.com", "Bob", Some("Full-stack developer")),
        (3i64, "charlie@example.com", "Charlie", None::<&str>),
    ];

    for (id, email, name, bio) in users {
        client
            .execute(
                "INSERT INTO users (id, email, name, bio) VALUES ($1, $2, $3, $4)",
                &[&id, &email, &name, &bio],
            )
            .await?;
        println!("  Created user: {} <{}>", name, email);
    }

    // Seed posts
    let posts = [
        (
            1i64,
            1i64,
            "Getting Started with Rust",
            "Rust is a systems programming language...",
            true,
        ),
        (
            2i64,
            1i64,
            "Advanced Rust Patterns",
            "Let's explore some advanced patterns...",
            true,
        ),
        (
            3i64,
            2i64,
            "Building Web Apps",
            "Modern web development with Rust...",
            true,
        ),
        (
            4i64,
            2i64,
            "Draft: Performance Tips",
            "Some tips I'm still working on...",
            false,
        ),
        (5i64, 3i64, "Hello World", "My first post!", true),
    ];

    for (id, author_id, title, body, published) in posts {
        client
            .execute(
                "INSERT INTO posts (id, author_id, title, body, published) VALUES ($1, $2, $3, $4, $5)",
                &[&id, &author_id, &title, &body, &published],
            )
            .await?;
        println!("  Created post: \"{}\" (published: {})", title, published);
    }

    println!("\nSeeding complete!");
    println!("  3 users");
    println!("  5 posts");

    Ok(())
}
