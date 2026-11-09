mod x11;

use crate::slog::*;

use super::Window;
use std::{
    ffi::{CString, c_ulong},
    time::Duration,
};

// TODO: error handling
pub fn open_window() -> Option<impl Window> {
    let display = unsafe { x11::XOpenDisplay(std::ptr::null()) };
    if display.is_null() {
        error!(target: open_window, "Couldn't open X11 display");
        return None;
    }

    let wm_delete =
        unsafe { x11::XInternAtom(display, x11::atoms::WM_DELETE_WINDOW.as_ptr(), x11::X_FALSE) };

    let screen = unsafe { x11::XDefaultScreen(display) };
    let root = unsafe { x11::XRootWindow(display, screen) };
    let window = unsafe {
        x11::XCreateSimpleWindow(
            display,
            root,
            100,
            100,
            400,
            300,
            1,
            x11::XBlackPixel(display, screen),
            x11::XWhitePixel(display, screen),
        )
    };

    unsafe {
        let mut wm_protocols = [wm_delete];
        x11::XSetWMProtocols(
            display,
            window,
            wm_protocols.as_mut_ptr(),
            wm_protocols.len() as _,
        );
        x11::XSelectInput(display, window, x11::EXPOSURE_MASK | x11::KEY_PRESS_MASK);
        x11::XMapWindow(display, window);
    }

    unsafe {
        let title = CString::new(crate::cfg::WINDOW_TITLE).unwrap();
        x11::XStoreName(display, window, title.as_ptr());
    }

    Some(X11Window {
        display,
        window,
        atom_wm_delete: wm_delete,
        keep_running: true,
    })
}

struct X11Window {
    display: *mut x11::Display,
    window: x11::Window,
    /* atoms */
    atom_wm_delete: x11::Atom,
    /* internal state */
    keep_running: bool,
}

impl Window for X11Window {
    fn is_open(&self) -> bool {
        self.keep_running
    }

    fn handle_events(&mut self) {
        let mut event = x11::XEvent { pad: [0; 24] };
        while unsafe { x11::XPending(self.display) > 0 } {
            unsafe { x11::XNextEvent(self.display, &mut event) };

            let event_type = unsafe { event.event_type };
            debug!(target: handle_events, "New event: {}", event_type);
            match event_type {
                x11::events::KEY_PRESS => (),      // TODO
                x11::events::KEY_RELEASE => (),    // TODO
                x11::events::BUTTON_PRESS => (),   // TODO
                x11::events::BUTTON_RELEASE => (), // TODO
                x11::events::MOTION_NOTIFY => (),  // TODO
                x11::events::ENTER_NOTIFY => (),   // TODO
                x11::events::LEAVE_NOTIFY => (),   // TODO
                x11::events::CLIENT_MESSAGE => {
                    let client = unsafe { event.client };
                    if client.data[0] as c_ulong == self.atom_wm_delete {
                        self.keep_running = false;
                    }
                }
                _ => (),
            }
        }
    }

    fn sleep(&self, time: Duration) {
        let mut pfds = [libc::pollfd {
            fd: unsafe { x11::XConnectionNumber(self.display) },
            events: libc::POLLIN,
            revents: 0,
        }];

        unsafe { libc::poll(pfds.as_mut_ptr(), pfds.len() as _, time.as_millis() as _) };
    }
}

impl Drop for X11Window {
    fn drop(&mut self) {
        unsafe {
            x11::XDestroyWindow(self.display, self.window);
            x11::XCloseDisplay(self.display);
        }
    }
}
