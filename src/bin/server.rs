#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

use skeleton::utils::{thread_pool::RayonThreadPool, server::Server, error::{Error, Result}};
use skeleton::engine::{Engine, EngineAdd};
use log::LevelFilter;
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
    addr: SocketAddr
}

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let mut opt = Opt::from_args();
    let res = run(opt);
    if let Err(e) = res {
        error!("{}", e);
        exit(1);
    }
}

fn run(opt: Opt) -> Result<()> {
    info!("Listening on {}", opt.addr);

    let concurrency = num_cpus::get() as u32;
    run_with::<EngineAdd::<RayonThreadPool>>(EngineAdd::<RayonThreadPool>::new(concurrency)?, opt.addr)
}

pub fn run_with<E: Engine>(engine: E, addr: SocketAddr) -> Result<()> {
    let server = Server::new(engine);
    server.run(addr)
}
