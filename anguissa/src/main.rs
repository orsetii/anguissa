use display::cli::log_frame;
use interface::CAN::{device::Device, init_vcan};
use std::{path::PathBuf};
use tracing::{info, warn};
use tracing_subscriber;
use clap::{ArgAction, arg, value_parser, Command};

mod display;
mod error;
mod interface;

pub use error::{Error, Result};

#[derive(Clone, Debug)]
pub struct CliArgs {
    pub debug_level: u8,
    pub port_name: String,
pub generate_random_traffic: bool
}


fn main() -> crate::Result<()> {
    // TODO make this configurable
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();
    let args = process_args();

    let portname = args.port_name.clone();

    if args.generate_random_traffic {
        std::thread::spawn(move || {
            std::process::Command::new(format!("cangen")).arg(portname).spawn().expect("failed to generate random traffic")
        });
    }

    let dev = Device::new(&args.port_name).unwrap();

    // First, print out the headers for the forthcoming data
    println!("[   Time   ] [ ID] [Flags]   [             Data             ]");
    println!("-------------------------------------------------------------");
    loop {
        match dev.read_frame() {
            Ok(frame) => {
                log_frame(frame)
            }
            Err(e) => {
                warn!("Error reading frame: {}", e);
            }
        }
    }

    Ok(())
}


fn process_args() -> CliArgs {

    let matches = clap::command!() // requires `cargo` feature
        .arg(
            arg!(
                -p --port <PORT> "CAN port name, defaults to vcan0"
            )
            .default_value("vcan0")
            .value_parser(value_parser!(String)).id("port"),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .arg(arg!(
            --genrandom  "Generate random CAN traffic on the port"
        ).action(ArgAction::SetTrue))
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .subcommand(
            Command::new("setup")
                .about("Setup parts of your system")
                .arg_required_else_help(true)
                .arg(arg!(--virtualport "Setup a virtual port for use").action(ArgAction::SetTrue)),
        )
        .get_matches();


     let generate_random_traffic = matches.get_flag("genrandom");

     let port_name = matches.get_one::<String>("port")
                                .map(|s| s.to_string())
                                .expect("no port name supplied");




    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    let debug_level = matches
        .get_one::<u8>("debug")
        .or(Some(&0)).unwrap();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    if let Some(matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.get_flag("list") {
            // "$ myapp test -l" was run
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    };


    if let Some(matches) = matches.subcommand_matches("setup") {
        // "$ myapp test" was run
        if matches.get_flag("virtualport") {
            init_vcan().expect("Unable to initialize virtual CAN port");
            println!("Successfully setup virtual CAN port");
            std::process::exit(0);
        } else {
        // help will be printed in this case.
        }
    };


    CliArgs{
        debug_level: *debug_level,
        port_name,
        generate_random_traffic
    }
}