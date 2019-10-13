use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
  Command1 { arg1: u32 },
  Command2 { arg1: u32, arg2: u32 },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
  Command1(u32),
  Command2(u32),
  Err(String),
}
