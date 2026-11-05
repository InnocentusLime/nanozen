mod platform;

use platform::*;

fn main() {
    let wnd = open_window();
    close_window(wnd);
}
