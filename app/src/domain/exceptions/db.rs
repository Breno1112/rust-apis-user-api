pub struct DatabaseException {
    pub message: String,
    pub kind: DatabaseExceptionType
}

#[derive(PartialEq)]
pub enum DatabaseExceptionType {
    EntityAlreadyExists,
    UnknownError
}