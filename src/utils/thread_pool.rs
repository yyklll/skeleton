use std::sync::Arc;

use crate::utils::error::{Error, Result};

pub trait ThreadPool: Clone + Send + 'static {
  fn new(threads: u32) -> Result<Self>
  where
    Self: Sized;

  fn spawn<F>(&self, job: F)
  where
    F: FnOnce() + Send + 'static;
}

#[derive(Clone)]
pub struct RayonThreadPool(Arc<rayon::ThreadPool>);

impl ThreadPool for RayonThreadPool {
  fn new(threads: u32) -> Result<Self> {
    let pool = rayon::ThreadPoolBuilder::new()
      .num_threads(threads as usize)
      .panic_handler(|_| {
        error!("panic catched");
      })
      .build()
      .map_err(|e| Error::StringError(format!("{}", e)))?;
    Ok(RayonThreadPool(Arc::new(pool)))
  }

  fn spawn<F>(&self, job: F)
  where
    F: FnOnce() + Send + 'static,
  {
    self.0.spawn(job)
  }
}
