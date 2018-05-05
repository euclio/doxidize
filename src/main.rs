extern crate doxidize;

#[macro_use]
extern crate configure;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::path::PathBuf;

use slog::Drain;
use structopt::StructOpt;

use doxidize::{Config, error};

#[derive(StructOpt, Debug)]
#[structopt(name = "doxidize", about = "Excellent documentation tooling for Rust")]
struct Opt {
    #[structopt(subcommand)]
    command: Option<Command>,

    #[structopt(long = "manifest-path",
                help = "Path to Cargo.toml",
                default_value = "Cargo.toml",
                parse(from_os_str))]
    manifest_path: PathBuf,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "build")]
    Build,
    #[structopt(name = "clean")]
    Clean,
    #[structopt(name = "publish")]
    Publish,
    #[structopt(name = "serve")]
    Serve,
    #[structopt(name = "init")]
    Init,
    #[structopt(name = "update")]
    Update,
}

fn main() {
    use_default_config!();

    let doxidize_version = env!("CARGO_PKG_VERSION");

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!("version" => doxidize_version));

    let opts = Opt::from_args();

    let config = Config::new(opts.manifest_path);

    let result = match opts.command {
        Some(Command::Build) => doxidize::ops::build(&config, &log),
        Some(Command::Clean) => doxidize::ops::clean(&config, &log),
        Some(Command::Publish) => doxidize::ops::publish(&config, &log),
        Some(Command::Serve) => doxidize::ops::serve(config, &log),
        Some(Command::Init) => doxidize::ops::init(&config, &log),
        Some(Command::Update) => doxidize::ops::update(&config, &log),
        None => doxidize::ops::init(&config, &log),
    };

    if let Err(err) = result {
        eprintln!("error: {}", err);

        if err.downcast_ref::<error::InitializedProject>().is_some() {
            eprintln!("help: try removing the docs directory and/or Doxidize.toml and try again");
        } else if err.downcast_ref::<error::UninitializedProject>().is_some() {
            eprintln!("help: try `doxidize init`");
        }

        std::process::exit(1);
    }
}
