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

impl Connection for PostgresPersistence<'_> {
    async fn query() {}

    async fn transaction_start(&mut self) -> Result<()> {
        let tx_arc = Arc::clone(&self.transaction);

        // limit lock acquisition in block scope
        {
            // acquiring lock
            let mut tx_changer_lock = tx_arc.lock().unwrap();
            // dereferencing MutexGuard (impl Deref)
            *tx_changer_lock = Some(self.pool.begin().await?);
        } // tx_changer_lock drops here

        Ok(())
    }

    async fn transaction_commit(self) -> Result<()> {
        let tx_arc = Arc::clone(&self.transaction);

        let tx_option = {
            let mut tx_changer_lock = tx_arc.lock().unwrap();
            tx_changer_lock.deref_mut().take()
        };

        if let Some(tx) = tx_option {
            tx.commit().await?;
        }

        Ok(())
    }

    async fn transaction_rollback(&mut self) -> Result<()> {
        Ok(())
    }
}

impl AccountsActions for PostgresPersistence<'_> {
    async fn create_account() {}
    async fn read_account() {}
    async fn update_account() {}
    async fn delete_account() {}
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use persistence::postgres::PostgresPersistence;
//
//     #[tokio::test]
//     async fn test_instantiation() {
//         let db_future = PostgresPersistence::new();
//         assert!(db_future.await.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_get_connection() {
//         let db_future = PostgresPersistence::new();
//         assert!(db_future.await.is_ok());
//     }
// }

