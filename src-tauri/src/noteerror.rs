use serde::{Serialize, Deserialize};
use core::fmt::{Display};

#[derive(Serialize, Deserialize,Debug)]
pub struct NoteError {
    message: String
}

impl NoteError {
  pub fn new(message: &str) -> Self {
      NoteError { message: message.into() }
  }
}

impl From<NoteError> for String {
  fn from(value: NoteError) -> Self {
      value.message.clone()
  }
}

impl<E: Display> From<E> for NoteError {
  fn from(value: E) -> Self {
      NoteError { message: format!("{}", value) }
  }
}

pub type NoteResult<T> = Result<T, NoteError>;