//! Schema definitions for my-app.
//!
//! This crate defines the database schema using facet reflection.

use facet::Facet;

/// A user in the system.
#[derive(Debug, Clone, Facet)]
#[facet(dibs::table = "users")]
pub struct User {
    /// Primary key
    #[facet(dibs::pk)]
    pub id: i64,

    /// User's email address
    #[facet(dibs::unique)]
    pub email: String,

    /// Display name
    pub name: String,

    /// Optional bio
    pub bio: Option<String>,

    /// When the user was created
    #[facet(dibs::default = "now()")]
    pub created_at: String,
}

/// A blog post.
#[derive(Debug, Clone, Facet)]
#[facet(dibs::table = "posts")]
pub struct Post {
    /// Primary key
    #[facet(dibs::pk)]
    pub id: i64,

    /// Author of the post
    #[facet(dibs::fk = "users.id")]
    pub author_id: i64,

    /// Post title
    pub title: String,

    /// Post content
    pub body: String,

    /// Whether the post is published
    #[facet(dibs::default = "false")]
    pub published: bool,

    /// When the post was created
    #[facet(dibs::default = "now()")]
    pub created_at: String,
}
