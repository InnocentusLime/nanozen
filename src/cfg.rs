use std::time::Duration;

pub static WINDOW_TITLE: &str = "Nanozen";
pub const FRAMES_PER_SECOND: u32 = 60;

pub const FRAME_DURATION: Duration = match Duration::from_secs(1).checked_div(FRAMES_PER_SECOND) {
    Some(x) => x,
    None => panic!("frame duration: div failed"),
};
pub const DELTA_TIME_CAP: Duration = match FRAME_DURATION.checked_mul(10) {
    Some(x) => x,
    None => panic!("delta time cap: mul failed"),
};
