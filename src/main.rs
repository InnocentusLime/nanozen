mod cfg;
mod platform;
mod slog;

use std::process::ExitCode;
use std::time::{Duration, Instant};

use platform::*;

fn main() -> ExitCode {
    let Some(mut wnd) = open_window() else {
        return ExitCode::FAILURE;
    };

    let mut accum = Duration::ZERO;
    let mut prev = Instant::now();
    while wnd.is_open() {
        wnd.handle_events();

        let now = Instant::now();
        let dt = prev.duration_since(now);
        prev = now;

        accum += std::cmp::min(dt, cfg::DELTA_TIME_CAP);
        while accum > cfg::FRAME_DURATION {
            /* TODO: update */
            accum = accum.saturating_sub(cfg::FRAME_DURATION);
        }
        /* TODO: redraw */

        let sleep_time = cfg::FRAME_DURATION
            .saturating_sub(accum)
            .saturating_sub(Duration::from_micros(100));
        wnd.sleep(sleep_time);
    }

    ExitCode::SUCCESS
}
