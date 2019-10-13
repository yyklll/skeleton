use crate::utils::{
  error::{Error, Result},
  thread_pool::ThreadPool,
};

use tokio::prelude::*;
use tokio::sync::oneshot;

pub trait Engine: Clone + Send + 'static {
  /// increment arg1 by 1
  fn command1(&self, arg1: u32) -> Box<Future<Item = u32, Error = Error> + Send>;

  /// arg1 + arg2
  fn command2(&self, arg1: u32, arg2: u32) -> Box<Future<Item = u32, Error = Error> + Send>;
}

#[derive(Clone)]
pub struct EngineAdd<P: ThreadPool> {
  pool: P,
  // add computation plugin here
}

impl<P: ThreadPool> EngineAdd<P> {
  pub fn new(concurrency: u32) -> Result<Self> {
    let pool = P::new(concurrency)?;
    Ok(EngineAdd { pool })
  }
}

impl<P: ThreadPool> Engine for EngineAdd<P> {
  fn command1(&self, arg1: u32) -> Box<Future<Item = u32, Error = Error> + Send> {
    let (tx, rx) = oneshot::channel();
    self.pool.spawn(move || {
      let res = Ok(arg1 + 1);
      if tx.send(res).is_err() {
        eprintln!("Receiving end is dropped");
      }
    });
    Box::new(
      rx.map_err(|e| Error::StringError(format!("{}", e)))
        .flatten(),
    )
  }

  fn command2(&self, arg1: u32, arg2: u32) -> Box<Future<Item = u32, Error = Error> + Send> {
    let (tx, rx) = oneshot::channel();
    self.pool.spawn(move || {
      let res = Ok(arg1 + arg2);
      if tx.send(res).is_err() {
        eprintln!("Receiving end is dropped");
      }
    });
    Box::new(
      rx.map_err(|e| Error::StringError(format!("{}", e)))
        .flatten(),
    )
  }
}
