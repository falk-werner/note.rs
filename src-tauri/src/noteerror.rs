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

  pub fn to_string(&self) -> String {
    self.message.clone()
  }
}

impl<E: Display> From<E> for NoteError {
  fn from(value: E) -> Self {
      NoteError { message: value.to_string() }
  }
}

pub type NoteResult<T> = Result<T, NoteError>;