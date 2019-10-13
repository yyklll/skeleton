use crate::utils::common::{Request, Response};
use crate::utils::error::Error;
use std::net::SocketAddr;
use tokio::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use tokio::io::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio_serde_json::{ReadJson, WriteJson};

pub struct Client {
  read_json: ReadJson<FramedRead<ReadHalf<TcpStream>, LengthDelimitedCodec>, Response>,
  write_json: WriteJson<FramedWrite<WriteHalf<TcpStream>, LengthDelimitedCodec>, Request>,
}

impl Client {
  pub fn connect(addr: SocketAddr) -> impl Future<Item = Self, Error = Error> {
    TcpStream::connect(&addr)
      .map(|conn| {
        let (read_half, write_half) = conn.split();
        let read_json = ReadJson::new(FramedRead::new(read_half, LengthDelimitedCodec::new()));
        let write_json = WriteJson::new(FramedWrite::new(write_half, LengthDelimitedCodec::new()));
        Client {
          read_json,
          write_json,
        }
      })
      .map_err(|e| e.into())
  }

  pub fn command1(self, arg1: u32) -> impl Future<Item = (u32, Self), Error = Error> {
    self
      .send_request(Request::Command1 { arg1 })
      .and_then(move |(resp, client)| match resp {
        Some(Response::Command1(value)) => Ok((value, client)),
        Some(Response::Err(msg)) => Err(Error::StringError(msg)),
        Some(_) => Err(Error::StringError("Invalid response".to_owned())),
        None => Err(Error::StringError("No response received".to_owned())),
      })
  }

  pub fn command2(self, arg1: u32, arg2: u32) -> impl Future<Item = (u32, Self), Error = Error> {
    self
      .send_request(Request::Command2 { arg1, arg2 })
      .and_then(move |(resp, client)| match resp {
        Some(Response::Command2(value)) => Ok((value, client)),
        Some(Response::Err(msg)) => Err(Error::StringError(msg)),
        Some(_) => Err(Error::StringError("Invalid response".to_owned())),
        None => Err(Error::StringError("No response received".to_owned())),
      })
  }

  fn send_request(
    self,
    req: Request,
  ) -> impl Future<Item = (Option<Response>, Self), Error = Error> {
    let read_json = self.read_json;
    self
      .write_json
      .send(req)
      .and_then(move |write_json| {
        read_json
          .into_future()
          .map(move |(resp, read_json)| {
            let client = Client {
              read_json,
              write_json,
            };
            (resp, client)
          })
          .map_err(|(err, _)| err)
      })
      .map_err(|e| e.into())
  }
}
