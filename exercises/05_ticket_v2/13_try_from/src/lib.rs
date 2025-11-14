use std::{error::Error as StdError, fmt};
// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.
#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug)]
pub enum Error {
    InvalidStatus,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidStatus => write!(f, "invalid status"),
        }
    }
}

impl StdError for Error {}

impl TryFrom<String> for Status {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.to_lowercase().as_str() {
            "todo" => Status::ToDo,
            "inprogress" => Status::InProgress,
            "done" => Status::Done,
            _ => return Err(Self::Error::InvalidStatus),
        })
    }
}

impl TryFrom<&str> for Status {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Status::try_from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
