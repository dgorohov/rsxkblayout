use xlib;
use crate::errors::X11Error;
use std::{ptr};
use std::option::Option;
use std::ffi::CStr;
use std::alloc::{alloc, Layout};

#[derive(Clone, Copy)]
pub struct Display {
    raw: *mut xlib::Display,
}

impl Display {
    pub fn open() -> Result<Self, X11Error> {
        let display = unsafe {
            xlib::XOpenDisplay(ptr::null_mut())
        };
        if display.is_null() {
            Err("open display")?;
        }
        Ok(Display { raw: display })
    }

    pub fn close(&self) {
        unsafe {
            xlib::XCloseDisplay(self.raw)
        };
    }
}

pub struct KeyboardState {
    d: Display
}

impl<'a> KeyboardState {

    pub fn new(disp: Display) -> Self {
        Self {
            d: disp
        }
    }

    pub fn get_current_idx(&self) -> usize {
        let mut state = xlib::XkbStateRec{
            group: 0,
            locked_group: 0,
            base_group: 0,
            latched_group: 0,
            mods: 0,
            base_mods: 0,
            latched_mods: 0,
            locked_mods: 0,
            compat_state: 0,
            grab_mods: 0,
            compat_grab_mods: 0,
            lookup_mods: 0,
            compat_lookup_mods: 0,
            ptr_buttons: 0
        };
        unsafe {
            xlib::XkbGetState(
                self.d.raw,
                xlib::XkbUseCoreKbd,
                &mut state
            );
            state.group.into()
        }
    }

    pub fn get_short_name(&self, idx: usize) -> Option<&'a str> {
        let allocator = Layout::new::<xlib::XkbRF_VarDefsRec>();
        let layouts;
        unsafe {
            let xkb_rf_ptr = alloc(allocator) as *mut xlib::XkbRF_VarDefsRec;

            xlib::XkbRF_GetNamesProp(
                self.d.raw,
                ptr::null_mut(),
                xkb_rf_ptr
            );
            layouts = CStr::from_ptr((*xkb_rf_ptr).layout)
        }
        match layouts.to_str() {
            Ok(v) => {
                let mut splitted_layouts = v.split(",");
                splitted_layouts.nth(idx)
            },
            Err(_e) => None
        }
    }
}


pub struct XkbEventProcessor {
    disp: Display,
    xkb_event_type: i32
}

impl XkbEventProcessor {
    pub fn new(d: Display) -> Self {
        let mut v: i32 = 0;
        unsafe {
            xlib::XkbQueryExtension(
                d.raw,
                ptr::null_mut(),
                &mut v,
                ptr::null_mut(), ptr::null_mut(), ptr::null_mut()
            );
            xlib::XkbSelectEvents(
                d.raw,
                xlib::XkbUseCoreKbd,
                xlib::XkbAllEventsMask,
                xlib::XkbAllEventsMask
            );
        }
        XkbEventProcessor {
            disp: d,
            xkb_event_type: v
        }
    }

    pub fn get_next_event(&mut self) -> Option<xlib::XkbEvent> {
        let allocator = Layout::new::<xlib::XEvent>();
        unsafe {
            let x_evt_ptr = alloc(allocator) as *mut xlib::XEvent;
            xlib::XNextEvent(self.disp.raw, x_evt_ptr);
            
            if (*x_evt_ptr).type_ == self.xkb_event_type {
                Some(*(x_evt_ptr as *mut xlib::XkbEvent))
            } else {
                None
            }
        }
    }
}

