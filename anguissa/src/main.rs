use std::{fs::File, io::Read};
use socketcan::CANSocket;
use tracing::{info, warn};
use tracing_subscriber;

mod interface;
mod error;

pub use error::{Result, Error};

fn main() {
    // TODO make this configurable
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();



    let dev = CANSocket::open("vcan0").unwrap();

    loop {

        match dev.read_frame() {
            Ok(frame) => {
                info!( "Read Frame: {:x?}", frame.data());
            }
            , Err(e) => {
                warn!("Error reading frame: {}", e);
            }
        }
    }

}