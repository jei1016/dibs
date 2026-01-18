//! Database service binary for my-app.
//!
//! This binary is spawned by the dibs CLI and communicates via roam.

// Import schema types to register them with inventory
use my_app_db::{Post, User};

fn main() {
    // Touch the types so they're not dead code eliminated
    let _ = (std::any::type_name::<User>(), std::any::type_name::<Post>());

    // Run the dibs service (connects back to CLI via roam)
    dibs::run_service();
}
