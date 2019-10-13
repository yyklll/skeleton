#[macro_use]
extern crate log;

extern crate config;

use log::LevelFilter;
use skeleton::engine::{Engine, EngineAdd};
use skeleton::utils::{
  error::Result,
  server::Server,
  thread_pool::RayonThreadPool,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::process::exit;
use structopt::StructOpt;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";

#[derive(StructOpt, Debug)]
#[structopt(name = "server")]
struct Opt {
  #[structopt(
    long,
    help = "Sets the listening address",
    value_name = "IP:PORT",
    raw(default_value = "DEFAULT_LISTENING_ADDRESS"),
    parse(try_from_str)
  )]
  addr: SocketAddr,
}

fn main() {
  env_logger::builder().filter_level(LevelFilter::Info).init();
  let opt = Opt::from_args();

  // import configuration from file
  let mut settings = config::Config::default();
  settings.merge(config::File::with_name("Config")).unwrap();
  let configs_from_file = settings.try_into::<HashMap<String, String>>().unwrap();

  let res = run(opt);
  if let Err(e) = res {
    error!("{}", e);
    exit(1);
  }
}

fn run(opt: Opt) -> Result<()> {
  info!("Listening on {}", opt.addr);

  let concurrency = num_cpus::get() as u32;
  run_with::<EngineAdd<RayonThreadPool>>(EngineAdd::<RayonThreadPool>::new(concurrency)?, opt.addr)
}

pub fn run_with<E: Engine>(engine: E, addr: SocketAddr) -> Result<()> {
  let server = Server::new(engine);
  server.run(addr)
}
