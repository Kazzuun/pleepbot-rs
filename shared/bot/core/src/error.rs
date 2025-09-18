use crate::cooldown;

#[derive(Debug, Clone)]
pub enum BotError {
    CommandError(String),
    CommandOnCooldown,
    InsufficientPermissions(),
    // NetworkError(reqwest::Error), // Example: if you use HTTP requests
    // DatabaseError(sqlx::Error),   // Example: if you use a database
    // Other(String),
}

impl std::fmt::Display for BotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BotError::CommandError(msg) => write!(f, "Command error: {}", msg),
            BotError::CommandOnCooldown() => write!(f, "Command on cooldown"),
            BotError::InsufficientPermissions() => write!(f, "Insufficient permissions"),
            // BotError::NetworkError(err) => write!(f, "Network error: {}", err),
            // BotError::DatabaseError(err) => write!(f, "Database error: {}", err),
            // BotError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl std::error::Error for BotError {}
