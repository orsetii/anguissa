use socketcan::CANSocket;
use crate::Result;

use super::frame::Frame;

pub struct Device {
    socket: CANSocket,
}

impl Device {
    pub fn open(dev_path: &str) -> Result<Self> {

        Ok(Self {
            socket: CANSocket::open(dev_path)?,
        })
    }

    pub fn new(dev_path: &str) -> Result<Self> {
        return Self::open(dev_path);
    }

    pub fn read_frame(&self) -> Result<Frame> {
        match self.socket.read_frame() {
            Ok(f) => {
                unsafe {
                return Ok(std::mem::transmute::<socketcan::CANFrame, Frame>(f));
                }
            }
            Err(_e) => {
                return Err(crate::Error::InvalidFrame.into());
            }
        }

    }
}