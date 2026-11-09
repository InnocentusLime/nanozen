use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong};

pub const EXPOSURE_MASK: c_long = 1 << 15;
pub const KEY_PRESS_MASK: c_long = 1 << 0;

pub mod atoms {
    use std::ffi::CStr;

    pub static WM_DELETE_WINDOW: &'static CStr = must_zero_terminate(b"WM_DELETE_WINDOW\0");

    const fn must_zero_terminate(s: &'static [u8]) -> &'static CStr {
        let x = CStr::from_bytes_with_nul(s);
        match x {
            Ok(x) => x,
            Err(_) => panic!("not zero terminated"),
        }
    }
}

pub mod events {
    use std::os::raw::c_int;

    pub const KEY_PRESS: c_int = 2; // Key down
    pub const KEY_RELEASE: c_int = 3; // Key up
    pub const BUTTON_PRESS: c_int = 4; // Mouse button down
    pub const BUTTON_RELEASE: c_int = 5; // Mouse button up
    pub const MOTION_NOTIFY: c_int = 6; // Mouse pointer moved
    pub const ENTER_NOTIFY: c_int = 7; // Mouse pointer entered
    pub const LEAVE_NOTIFY: c_int = 8; // Mouse pointer left
    pub const CLIENT_MESSAGE: c_int = 33; // Client communication message
}

pub type XID = c_ulong;
pub type Atom = XID;
pub type Window = XID;
pub type Bool = c_int;

pub const X_TRUE: Bool = 1;
pub const X_FALSE: Bool = 0;

#[repr(C)]
pub struct Display {
    _private: [u8; 0],
}

#[repr(C)]
pub union XEvent {
    pub event_type: c_int,
    pub client: XClientMessageEvent,
    pub pad: [c_long; 24],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XClientMessageEvent {
    pub event_type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut Display,
    pub window: c_ulong,
    pub message_type: Atom,
    pub format: c_int,
    pub data: [c_long; 5],
}

#[link(name = "X11")]
unsafe extern "C" {
    pub fn XOpenDisplay(name: *const c_char) -> *mut Display;
    pub fn XDefaultScreen(display: *mut Display) -> c_int;
    pub fn XRootWindow(display: *mut Display, screen: c_int) -> Window;
    pub fn XSync(display: *mut Display, discard: Bool);

    pub fn XBlackPixel(display: *mut Display, screen: c_int) -> c_ulong;
    pub fn XWhitePixel(display: *mut Display, screen: c_int) -> c_ulong;

    pub fn XCreateSimpleWindow(
        display: *mut Display,
        parent: c_ulong,
        x: c_int,
        y: c_int,
        width: c_uint,
        height: c_uint,
        border_width: c_uint,
        border: c_ulong,
        background: c_ulong,
    ) -> Window;

    pub fn XStoreName(display: *mut Display, window: c_ulong, name: *const c_char) -> c_int;

    pub fn XMapWindow(display: *mut Display, window: c_ulong) -> c_int;
    pub fn XSelectInput(display: *mut Display, window: c_ulong, mask: c_long) -> c_int;

    pub fn XNextEvent(display: *mut Display, event: *mut XEvent) -> c_int;
    pub fn XDestroyWindow(display: *mut Display, window: c_ulong);
    pub fn XCloseDisplay(display: *mut Display) -> c_int;

    pub fn XInternAtom(display: *mut Display, name: *const c_char, only_if_exists: Bool) -> Atom;

    pub fn XSetWMProtocols(
        display: *mut Display,
        window: c_ulong,
        protocols: *mut Atom,
        count: c_int,
    ) -> c_int;

    pub fn XPending(display: *mut Display) -> c_int;

    pub fn XConnectionNumber(display: *mut Display) -> c_int;
}
