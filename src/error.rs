use core::num;
use std::error::Error;
use std::fmt::Display;
use std::io;


#[derive(Debug)]
pub enum ChatError {
	WrongIdError(usize),
	DecodeMessageError,
	IoError(io::Error),
	ParseInt(num::ParseIntError),
}

impl Error for ChatError {
	fn description(&self) -> &str {
		match self {
			ChatError::WrongIdError(_) => "wrong command id error",
			ChatError::DecodeMessageError => "decode message error",
			ChatError::IoError(_) => "io error",
			ChatError::ParseInt(_) => "parse int error",
		}
	}
}

impl Display for ChatError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ChatError::WrongIdError(id) => write!(f,"Wrong command id:{} error",id),
			ChatError::DecodeMessageError => write!(f,"decode message error"),
			ChatError::IoError(err) => write!(f,"io error {}",err),
			ChatError::ParseInt(err) => write!(f,"parse int error {err}"),
		}
	}
}


impl From<io::Error> for ChatError {
	fn from(error: io::Error) -> Self {
		ChatError::IoError(error)
	}
}


impl From<num::ParseIntError> for ChatError {
	fn from(error: num::ParseIntError) -> Self {
		ChatError::ParseInt(error)
	}
}

