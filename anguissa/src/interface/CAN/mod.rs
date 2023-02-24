use std::process::Command;
pub mod device;
pub mod frame;



pub fn init_vcan() -> crate::Result<()> {
    const CAN_PORT: &str = "vcan0";
    Command::new("sudo").args(["modprobe", "vcan"]).output()?;
    Command::new("sudo").args(["ip", "link", "add", "dev", CAN_PORT, "type", "vcan"]).output()?;
    Command::new("sudo").args(["ip", "link", "set", "up", CAN_PORT, ]).output()?;

    Ok(())
}