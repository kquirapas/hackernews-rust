use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("Transaction already running")]
    TransactionAlreadyRunning,
}
