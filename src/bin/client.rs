use clap::AppSettings;
use skeleton::utils::{client::Client, error::Result};
use std::net::SocketAddr;
use std::process::exit;
use structopt::StructOpt;
use tokio::prelude::*;

#[derive(StructOpt, Debug)]
#[structopt(
  name = "client",
  raw(global_settings = "&[\
                         AppSettings::DisableHelpSubcommand,\
                         AppSettings::VersionlessSubcommands]")
)]
struct Opt {
  #[structopt(subcommand)]
  command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
  #[structopt(name = "command1", about = "arg1++")]
  Command1 {
    #[structopt(name = "arg1")]
    arg1: u32,

    #[structopt(
      long,
      help = "Sets the server address",
      value_name = "IP:PORT",
      default_value = "127.0.0.1:4000",
      parse(try_from_str)
    )]
    addr: SocketAddr,
  },
  #[structopt(name = "command2", about = "arg1 + arg2")]
  Command2 {
    #[structopt(name = "arg1")]
    arg1: u32,

    #[structopt(name = "arg2")]
    arg2: u32,
    
    #[structopt(
      long,
      help = "Sets the server address",
      value_name = "IP:PORT",
      default_value = "127.0.0.1:4000",
      parse(try_from_str)
    )]
    addr: SocketAddr,
  },
}

fn main() {
  let opt = Opt::from_args();
  if let Err(e) = run(opt) {
    eprintln!("{}", e);
    exit(1);
  }
}

fn run(opt: Opt) -> Result<()> {
  match opt.command {
    Command::Command1 { arg1, addr } => {
      let client = Client::connect(addr);
      if let (value, _) = client
        .and_then(move |client| client.command1(arg1))
        .wait()?
      {
        println!("{}", value);
      } else {
        eprintln!("error");
      }
    }
    Command::Command2 { arg1, arg2, addr } => {
      let client = Client::connect(addr);
      if let (value, _) = client
        .and_then(move |client| client.command2(arg1, arg2))
        .wait()?
      {
        println!("{}", value);
      } else {
        eprintln!("error");
      }
    }
  }
  Ok(())
}
