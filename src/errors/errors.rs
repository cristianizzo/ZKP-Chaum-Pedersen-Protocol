use tonic::Status;
use log::error;

pub fn internal_server_error<T>(err: T) -> Status
    where
        T: std::fmt::Display,
{
    error!("Internal server error: {}", err);
    Status::internal("Internal server error")
}

pub fn already_exists_error<T>(err: T) -> Status
    where
        T: std::fmt::Display,
{
    error!("Resource already exists: {}", err);
    Status::already_exists("Resource already exists")
}
