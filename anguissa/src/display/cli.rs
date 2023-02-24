//! The format of a CAN frame that is logged is as so:
//! [time] [id] [eff] [rtr] [err] [data]
use owo_colors::OwoColorize;
use crate::interface::CAN::frame::Frame;

pub fn log_frame(frame: Frame) {
    
    println!("{} [{:<3X}] [{}-{}-{}] : {:02x?}", chrono::Local::now().format( "%H:%M:%S%.3f"), frame.id().bright_blue(), bool_to_icon(frame.eff()), bool_to_icon(frame.rtr()), bool_to_icon(frame.err()),  frame.data());

}

#[inline]
fn bool_to_icon(x: bool) -> String {

    if x {
        return '✓'.green().to_string();
    } else {
        return 'x'.red().to_string();
    }
}