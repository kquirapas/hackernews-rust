use anyhow::{ensure, Context, Error, Result};
use async_trait::async_trait;
use sqlx::{
    postgres::{PgPool, PgPoolOptions, Postgres},
    Transaction,
};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

use crate::errors::ConnectionError;
use crate::ports::persistence::{Connection, Persistence};

pub struct PostgresPersistence<'a> {
    pub pool: PgPool,
    pub transaction: Arc<Mutex<Option<Transaction<'a, Postgres>>>>,
}

impl PostgresPersistence<'_> {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            pool: PgPoolOptions::new()
                .max_connections(5)
                .connect("postgres://superuser:superpassword@0.0.0.0:5432/postgres")
                .await
                .with_context(|| "Failed to connect to Postgres DB")?,
            transaction: Arc::new(Mutex::new(None)),
        })
    }
}

impl Persistence for PostgresPersistence<'_> {}

#[async_trait]
impl Connection for PostgresPersistence<'_> {
    async fn transaction_start(&mut self) -> Result<()> {
        let shared_tx_ref = Arc::clone(&self.transaction);

        let new_tx = self.pool.begin().await?;

        // limit lock acquisition in block scope
        {
            // acquiring lock
            let mut lock = shared_tx_ref.lock().unwrap();
            // dereferencing MutexGuard (impl DerefMut)
            let tx_option = lock.deref_mut();
            // Ensure no other Transactions already started
            ensure!(
                tx_option.is_none(),
                ConnectionError::TransactionAlreadyRunning
            );

            *tx_option = Some(new_tx)
        };

        Ok(())
    }

    async fn transaction_commit(&self) -> Result<()> {
        let shared_tx_ref = Arc::clone(&self.transaction);

        let shared_tx = {
            let mut lock = shared_tx_ref.lock().unwrap();
            lock.deref_mut().take()
        };

        if let Some(tx) = shared_tx {
            tx.commit().await?;
        }

        Ok(())
    }

    async fn transaction_rollback(&mut self) -> Result<()> {
        let shared_tx_ref = Arc::clone(&self.transaction);

        let shared_tx = {
            let mut lock = shared_tx_ref.lock().unwrap();
            lock.deref_mut().take()
        };

        if let Some(tx) = shared_tx {
            tx.rollback().await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::postgres::PostgresPersistence;

    #[tokio::test]
    async fn test_instantiation() {
        let db_future = PostgresPersistence::new();
        assert!(db_future.await.is_ok());
    }

    #[tokio::test]
    async fn test_transaction_start() {
        let mut db = PostgresPersistence::new().await.unwrap();
        assert!(db.transaction_start().await.is_ok());
    }

    #[tokio::test]
    async fn test_failure_transaction_already_running() {
        let mut db = PostgresPersistence::new().await.unwrap();

        // make sure creation of transaction was a success
        assert!(db.transaction_start().await.is_ok());

        // purposely invoke the error by starting 2nd time
        let ret = db.transaction_start().await.unwrap_err();

        // convert Error to a concrete type by downcasting
        let err_type = ret.downcast::<ConnectionError>().unwrap();

        // make sure it's the expected error type
        assert!(matches!(
            err_type,
            ConnectionError::TransactionAlreadyRunning
        ));
    }

    #[tokio::test]
    async fn test_transaction_commit() {
        let mut db = PostgresPersistence::new().await.unwrap();
        assert!(db.transaction_start().await.is_ok());
        // insert query
        assert!(db.transaction_commit().await.is_ok());

        // check that the query came through
    }

    #[tokio::test]
    async fn test_transaction_rollback() {
        let mut db = PostgresPersistence::new().await.unwrap();
        assert!(db.transaction_start().await.is_ok());
        // insert query
        assert!(db.transaction_rollback().await.is_ok());

        // check that the query didn't come through
    }
}
