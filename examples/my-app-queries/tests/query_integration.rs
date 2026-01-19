//! Integration tests for generated queries.
//!
//! These tests run against a real Postgres instance using dockside.

use dockside::{Container, containers};
use std::time::Duration;
use tokio_postgres::NoTls;

async fn create_postgres_container() -> (Container, tokio_postgres::Client) {
    let container = tokio::task::spawn_blocking(|| {
        let container = Container::run(containers::postgres("18", "postgres"))
            .expect("Failed to start Postgres container");

        container
            .wait_for_log(
                "database system is ready to accept connections",
                Duration::from_secs(30),
            )
            .expect("Postgres failed to become ready");

        let port = container
            .wait_for_port(5432, Duration::from_secs(5))
            .expect("Failed to connect to postgres port");

        (container, port)
    })
    .await
    .expect("spawn_blocking failed");

    let (container, port) = container;

    let connection_string = format!(
        "host=127.0.0.1 port={} user=postgres password=postgres dbname=postgres",
        port
    );

    let mut last_err = None;
    let mut client_and_conn = None;
    for _ in 0..30 {
        match tokio_postgres::connect(&connection_string, NoTls).await {
            Ok(c) => {
                client_and_conn = Some(c);
                break;
            }
            Err(e) => {
                last_err = Some(e);
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }
    let (client, connection) = client_and_conn
        .ok_or_else(|| last_err.unwrap())
        .expect("Failed to connect to Postgres after retries");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    (container, client)
}

async fn setup_schema(client: &tokio_postgres::Client) {
    // Create the product table matching my-app-db schema
    client
        .execute(
            r#"
            CREATE TABLE "product" (
                "id" BIGSERIAL PRIMARY KEY,
                "handle" TEXT NOT NULL UNIQUE,
                "status" TEXT NOT NULL DEFAULT 'draft',
                "active" BOOLEAN NOT NULL DEFAULT true,
                "metadata" TEXT,
                "created_at" TIMESTAMPTZ NOT NULL DEFAULT now(),
                "updated_at" TIMESTAMPTZ NOT NULL DEFAULT now(),
                "deleted_at" TIMESTAMPTZ
            )
            "#,
            &[],
        )
        .await
        .expect("Failed to create product table");
}

async fn insert_test_data(client: &tokio_postgres::Client) {
    // Insert test products with various statuses
    let products = [
        ("widget-a", "published", true),
        ("widget-b", "published", true),
        ("gadget-x", "published", false), // inactive
        ("prototype-z", "draft", true),
        ("old-product", "archived", true),
    ];

    for (handle, status, active) in products {
        client
            .execute(
                r#"INSERT INTO "product" ("handle", "status", "active") VALUES ($1, $2, $3)"#,
                &[&handle, &status, &active],
            )
            .await
            .expect("Failed to insert test product");
    }

    // Insert a soft-deleted product
    client
        .execute(
            r#"INSERT INTO "product" ("handle", "status", "active", "deleted_at") VALUES ($1, $2, $3, now())"#,
            &[&"deleted-product", &"published", &true],
        )
        .await
        .expect("Failed to insert deleted product");
}

#[tokio::test]
async fn test_all_products() {
    let (_container, client) = create_postgres_container().await;
    setup_schema(&client).await;
    insert_test_data(&client).await;

    let results = my_app_queries::all_products(&client).await.unwrap();

    // Should return all non-deleted products (5 total)
    assert_eq!(results.len(), 5, "Expected 5 non-deleted products");

    // Results should be ordered by created_at DESC (most recent first)
    // Since we inserted in order, last inserted should be first
    let handles: Vec<_> = results.iter().map(|p| p.handle.as_str()).collect();
    assert!(handles.contains(&"widget-a"));
    assert!(handles.contains(&"widget-b"));
    assert!(handles.contains(&"gadget-x"));
    assert!(handles.contains(&"prototype-z"));
    assert!(handles.contains(&"old-product"));

    // Should NOT contain deleted product
    assert!(!handles.contains(&"deleted-product"));
}

#[tokio::test]
async fn test_active_products() {
    let (_container, client) = create_postgres_container().await;
    setup_schema(&client).await;
    insert_test_data(&client).await;

    let results = my_app_queries::active_products(&client).await.unwrap();

    // Should only return published AND active products
    // widget-a: published, active ✓
    // widget-b: published, active ✓
    // gadget-x: published, inactive ✗
    // prototype-z: draft, active ✗
    // old-product: archived, active ✗
    assert_eq!(results.len(), 2, "Expected 2 active published products");

    let handles: Vec<_> = results.iter().map(|p| p.handle.as_str()).collect();
    assert!(handles.contains(&"widget-a"));
    assert!(handles.contains(&"widget-b"));

    // All should have status = published
    for result in &results {
        assert_eq!(result.status, "published");
    }
}

#[tokio::test]
async fn test_product_by_handle() {
    let (_container, client) = create_postgres_container().await;
    setup_schema(&client).await;
    insert_test_data(&client).await;

    // Find existing product
    let handle = "widget-a".to_string();
    let result = my_app_queries::product_by_handle(&client, &handle)
        .await
        .unwrap();

    assert!(result.is_some(), "Expected to find widget-a");
    let product = result.unwrap();
    assert_eq!(product.handle, "widget-a");
    assert_eq!(product.status, "published");
    assert!(product.active);
    // Note: created_at field removed due to jiff timestamp deserialization not yet supported

    // Find non-existent product
    let handle = "does-not-exist".to_string();
    let result = my_app_queries::product_by_handle(&client, &handle)
        .await
        .unwrap();
    assert!(result.is_none(), "Expected None for non-existent product");

    // Deleted product should not be found
    let handle = "deleted-product".to_string();
    let result = my_app_queries::product_by_handle(&client, &handle)
        .await
        .unwrap();
    assert!(
        result.is_none(),
        "Deleted product should not be found via query"
    );
}

#[tokio::test]
async fn test_search_products() {
    let (_container, client) = create_postgres_container().await;
    setup_schema(&client).await;
    insert_test_data(&client).await;

    // Search for "widget" - should match widget-a, widget-b
    let q = "%widget%".to_string();
    let results = my_app_queries::search_products(&client, &q).await.unwrap();

    assert_eq!(results.len(), 2, "Expected 2 products matching 'widget'");
    let handles: Vec<_> = results.iter().map(|p| p.handle.as_str()).collect();
    assert!(handles.contains(&"widget-a"));
    assert!(handles.contains(&"widget-b"));

    // Results should be ordered by handle ASC
    assert_eq!(results[0].handle, "widget-a");
    assert_eq!(results[1].handle, "widget-b");

    // Search for "gadget" - should match gadget-x
    let q = "%gadget%".to_string();
    let results = my_app_queries::search_products(&client, &q).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].handle, "gadget-x");

    // Search for non-matching pattern
    let q = "%nonexistent%".to_string();
    let results = my_app_queries::search_products(&client, &q).await.unwrap();
    assert!(results.is_empty());

    // Case-insensitive search (ILIKE)
    let q = "%WIDGET%".to_string();
    let results = my_app_queries::search_products(&client, &q).await.unwrap();
    assert_eq!(results.len(), 2, "ILIKE should be case-insensitive");
}
