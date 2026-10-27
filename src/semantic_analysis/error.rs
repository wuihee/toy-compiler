use thiserror::Error;

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("Some type error.")]
    Todo,
}
