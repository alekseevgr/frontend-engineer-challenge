use domain::shared::repository::error::RepositoryError;

pub fn map_sqlx_error_to_domain_error(e: sqlx::Error) -> RepositoryError {
    match e {
        sqlx::Error::Database(db_err) => {
            if let Some(constraint) = db_err.constraint()
                && (constraint.contains("name") || constraint.contains("unique"))
            {
                return RepositoryError::AlreadyExists;
            }
            RepositoryError::Internal(db_err.to_string())
        }
        sqlx::Error::RowNotFound => RepositoryError::NotFound,
        sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed => {
            RepositoryError::ConnectionError("Database pool error".to_string())
        }
        sqlx::Error::Io(err) => RepositoryError::ConnectionError(format!("IO error: {}", err)),
        sqlx::Error::Tls(err) => RepositoryError::ConnectionError(format!("TLS error: {}", err)),
        sqlx::Error::Protocol(err) => {
            RepositoryError::ConnectionError(format!("Protocol error: {}", err))
        }
        sqlx::Error::Configuration(err) => {
            RepositoryError::InvalidData(format!("Configuration error: {}", err))
        }
        sqlx::Error::Encode(err) => RepositoryError::InvalidData(format!("Encode error: {}", err)),
        sqlx::Error::Decode(err) => RepositoryError::InvalidData(format!("Decode error: {}", err)),
        sqlx::Error::TypeNotFound { type_name } => {
            RepositoryError::Internal(format!("Type not found: {}", type_name))
        }
        sqlx::Error::ColumnNotFound(column) => {
            RepositoryError::Internal(format!("Column not found: {}", column))
        }
        _ => RepositoryError::Internal(e.to_string()),
    }
}
