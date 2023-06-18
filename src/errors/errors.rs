use std::fmt;
use tonic::Status;

#[derive(Debug)]
pub enum AppError {
    DatabaseConnectionError(String),
    DatabaseError(sqlx::Error),
    UserAlreadyExists,
    UserDoesNotExist,
    BadCredentials,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::DatabaseConnectionError(ref err) => {
                write!(f, "Database Connection Error: {}", err)
            }
            AppError::DatabaseError(ref err) => write!(f, "Database error: {}", err),
            AppError::UserAlreadyExists => write!(f, "User Already Exists"),
            AppError::UserDoesNotExist => write!(f, "User Does Not Exist"),
            AppError::BadCredentials => write!(f, "Bad Credentials"),
        }
    }
}

impl From<AppError> for Status {
    fn from(error: AppError) -> Self {
        match error {
            AppError::DatabaseConnectionError(err) => Status::internal(err),
            AppError::DatabaseError(err) => Status::internal(err.to_string()),
            AppError::UserAlreadyExists => Status::already_exists("User already exists"),
            AppError::UserDoesNotExist => Status::not_found("User Does Not Exist"),
            AppError::BadCredentials => Status::unauthenticated("Bad Credentials"),
        }
    }
}
