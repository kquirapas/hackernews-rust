pub trait Connection {
    // -- snipped --
    async fn transaction_start(&mut self) -> Result<()>;
    // -- snipped --
}

pub struct PostgresPersistence<'a> {
    pub pool: PgPool,
    pub transaction: Arc<Mutex<Option<Transaction<'a, Postgres>>>>,
}

impl PostgresPersistence<'_> {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            transaction: Arc::new(Mutex::new(None)),
        })
    }
}

impl Connection for PostgresPersistence<'_> {
    // -- snipped --
    async fn transaction_commit(self) -> Result<()> {
        // explicitly create new arc
        let tx_arc = Arc::clone(&self.transaction);

        // shorten lock acquisition in block scope
        let tx_option = {
            let mut tx_changer_lock = tx_arc.lock().unwrap();
            tx_changer_lock.deref_mut().take()
        };

        if let Some(tx) = tx_option {
            tx.commit().await?;
        }

        Ok(())
    }
    // -- snipped --
}
