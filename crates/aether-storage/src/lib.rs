pub mod db;
pub mod schema;
pub mod search;

pub use db::Database;
pub use schema::initialize_schema;
pub use search::InstantSearchEngine;

pub fn storage_init() {
    tracing::info!("Aether Storage & Tantivy Instant Search Engine ready");
}
