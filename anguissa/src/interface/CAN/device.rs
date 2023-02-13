use socketcan::CANSocket;

pub struct Device {
    socket: CANSocket,
}

impl Device {
    pub fn open(dev_path: &str) -> crate::Result<Self> {

        Ok(Self {
            socket: CANSocket::open(dev_path)?,
        })
    }

    pub fn read_frame() -> crate::Result<
}