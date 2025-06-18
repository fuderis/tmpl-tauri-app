use macron::{ Display, From, Error };

// The result alias
pub type StdResult<T, E> = std::result::Result<T, E>;
pub type Result<T> = StdResult<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

// The error
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[from]
    #[display = "{0}"]
    String(String),
}
