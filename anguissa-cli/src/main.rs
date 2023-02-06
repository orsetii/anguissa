// Ignore some of clippy's warnings
#![allow(clippy::never_loop)]
#![allow(clippy::needless_return)]

use anyhow::Result;
use clap::Parser;
use futures_util::StreamExt;
use std::process;
use tokio_socketcan::CANSocket;


// Struct for the Can frame
#[derive(Debug, PartialEq)]
struct CanFrame {
    /// Can ID
    id: u32,
    /// Data section of the Can frame
    data: Vec<u8>,
}

// Struct for the CLI arguments
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct ClI {
    /// Can interface name. Eg; can0
    device: String,
    /// ID of the frame to match, Eg; 291
    frame_id: u32,
}

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    // Parse the CLI arguments
    let cli = ClI::parse();

    // Use the given CLI arguments
    let device = cli.device;
    let frame_id = cli.frame_id;

    // Check to see if the interface is up
    check_interface_is_up(&device);

    // print Can frame only when we find a match on the frame ID
    let data = get_can_frame_with_id(&device, frame_id)
        .await
        .ok_or("No CanFrame found")?;
    println!("------------------------------------------------------------------");
    println!("Matched can ID: {:?}", data.id);
    println!("Can data is: {:?}", data.data);
    println!("------------------------------------------------------------------");

    Ok(())
}

fn check_interface_is_up(device: &str) {
    println!("Checking if {:?} interface is up...", device);
    // Get all interface dev stats from procfs, this should never fail
    // as proctfs should at least contain the loopback interface, thus use unwrap()
    let dev_stats = procfs::net::dev_status().unwrap();

    // Search the dev stats output for the device name given to the function
    if dev_stats.contains_key(device) {
        match dev_stats.get(device) {
            // If we match on the given device name, print the stats of the interface
            Some(interface_stats) => println!(
                "Interface stats for: {:?},
             {:?}",
                device, interface_stats
            ),
            // This arm should never match
            _ => println!("Interface stats not found"),
        }
    } else {
        // If we don't have a match on the given interface name, exit the program with a helpful message
        println!("Interface: {:?} not found or down.", device);
        println!("Can not continue, exiting");
        // Exit the application, return code 1
        process::exit(1);
    }
}

async fn get_can_frame_with_id(device: &str, can_id: u32) -> Option<CanFrame> {
    // Try open the can interface, this should be up as we checked via the check_interface_is_up()
    // function, thus use unwrap()
    let mut socket_rx = CANSocket::open(device).expect("Unable to open Can Interface");

    // Keep getting Can frames
    while let Some(Ok(frame)) = socket_rx.next().await {
        let _can_id = frame.id();
        let _can_data = frame.data();

        // insert can frame data into our struct
        let can_frame = CanFrame {
            id: _can_id,
            data: _can_data.to_vec(),
        };

        // match on the frame ID and return the struct on match
        if frame.id() == can_id {
            return Some(can_frame);
        }
    }

    return None;
}
