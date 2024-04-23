//! Request Adapters for Hexagonal /
//! Ports and Adapters Architecture
use anyhow::Result;
use async_trait::async_trait;

// Persistence Super Trait
#[async_trait]
pub trait Connection {
    async fn transaction_start(&mut self) -> Result<()>;
    async fn transaction_commit(&self) -> Result<()>;
    async fn transaction_rollback(&mut self) -> Result<()>;
}

pub trait Persistence: Connection {}
