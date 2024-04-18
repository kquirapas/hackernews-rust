//! Persistence Adapters for Hexagonal /
//! Ports and Adapters Architecture
use anyhow::Result;
use async_trait::async_trait;

// Persistence Super Trait

#[async_trait]
pub trait Connection {
    async fn query(&self) -> Result<()>;
    async fn transaction_start(&mut self) -> Result<()>;
    async fn transaction_commit(&self) -> Result<()>;
    async fn transaction_rollback(&mut self) -> Result<()>;
}

// Persistence Super Trait
#[async_trait]
pub trait AccountsActions {
    // -- ACCOUNTS table methods --
    async fn create_account(&self) -> Result<()>;
    async fn read_account(&self) -> Result<()>;
    async fn update_account(&self) -> Result<()>;
    async fn delete_account(&self) -> Result<()>;
}

/// Sub trait of super traits Adapter and AccountActions
/// to guarantee the existence of both.
pub trait AccountsPersistence: Connection + AccountsActions {}
