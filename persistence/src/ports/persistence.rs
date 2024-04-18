//! Persistence Adapters for Hexagonal /
//! Ports and Adapters Architecture
use anyhow::Result;

// Persistence Super Trait
pub trait Connection {
    async fn query(&self) -> Result<()>;
    async fn transaction_start(&mut self) -> Result<()>;
    async fn transaction_commit(&self) -> Result<()>;
    async fn transaction_rollback(&mut self) -> Result<()>;
}

// Persistence Super Trait
pub trait AccountsActions {
    // -- ACCOUNTS table methods --
    async fn create_account();
    async fn read_account();
    async fn update_account();
    async fn delete_account();
}

/// Sub trait of super traits Adapter and AccountActions
/// to guarantee the existence of both.
pub trait AccountsPersistence: Connection + AccountsActions {}
