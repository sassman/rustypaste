//! **Rustypaste** is a minimal file upload/pastebin service.
#![warn(missing_docs, clippy::unwrap_used)]

/// Configuration file parser.
pub mod config;

/// Random URL generator.
pub mod random;

/// Server routes.
pub mod server;

/// HTTP headers.
pub mod header;

/// Auth handler.
pub mod auth;

/// Storage handler.
pub mod paste;

/// File metadata handler.
pub mod file;

/// Media type handler.
pub mod mime;

/// Helper functions.
pub mod util;
