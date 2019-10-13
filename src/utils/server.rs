use crate::engine::Engine;
use crate::utils::common::{Request, Response};
use crate::utils::error::{Error, Result};

use std::net::SocketAddr;
use tokio::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio_serde_json::{ReadJson, WriteJson};

pub struct Server<E: Engine> {
  engine: E,
}

impl<E: Engine> Server<E> {
  pub fn new(engine: E) -> Self {
    Server { engine }
  }

  /// Run the server listening on the given address
  pub fn run(self, addr: SocketAddr) -> Result<()> {
    let listener = TcpListener::bind(&addr)?;
    let server = listener
      .incoming()
      .map_err(|e| eprintln!("IO error: {}", e))
      .for_each(move |tcp| {
        let engine = self.engine.clone();
        serve(engine, tcp).map_err(|e| eprintln!("Error on serving client: {}", e))
      });
    tokio::run(server);
    Ok(())
  }
}

fn serve<E: Engine>(engine: E, tcp: TcpStream) -> impl Future<Item = (), Error = Error> {
  let (read_half, write_half) = tcp.split();
  let read_json = ReadJson::new(FramedRead::new(read_half, LengthDelimitedCodec::new()));
  let resp_stream = read_json
    .map_err(Error::from)
    .and_then(
      move |req| -> Box<dyn Future<Item = Response, Error = Error> + Send> {
        match req {
          Request::Command1 { arg1 } => Box::new(engine.command1(arg1).map(Response::Command1)),
          Request::Command2 { arg1, arg2 } => {
            Box::new(engine.command2(arg1, arg2).map(Response::Command2))
          }
        }
      },
    )
    .then(|resp| -> Result<Response> {
      match resp {
        Ok(resp) => Ok(resp),
        Err(e) => Ok(Response::Err(format!("{}", e))),
      }
    });
  let write_json = WriteJson::new(FramedWrite::new(write_half, LengthDelimitedCodec::new()));
  write_json
    .sink_map_err(Error::from)
    .send_all(resp_stream)
    .map(|_| ())
}
