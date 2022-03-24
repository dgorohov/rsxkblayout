#[macro_use]
extern crate quick_error;

mod errors;
mod x11;

use serde_json;

#[macro_use]
extern crate serde_derive;

use std::fmt;
use std::fmt::{Display, Formatter};
use crate::errors::X11Error;


struct Session {
    g: usize,
    d: x11::Display,
    k: x11::KeyboardState,
}

impl Session {
    fn new() -> Result<Self, X11Error> {
        match x11::Display::open() {
            Ok(d)  => Ok(Session{
                g: 0,
                d: d,
                k: x11::KeyboardState::new(d)
            }),
            Err(e) => Err(e)
        }
    }

    fn handle_xkb_event(&mut self, t: i32, idx: usize, force: bool) {
        if t == 2i32 && (force || self.g != idx) {
            self.g = idx;
            match self.k.get_short_name(idx) {
                Some(n) => println!("{}", Message::new(String::from(n))),
                None    => ()
            }
        }
   }
}

impl Drop for Session {
    fn drop(&mut self) {
        self.d.close()
    }
}

#[derive(Serialize, Debug)]
struct Message<'a> {
    full_text: String,
    color: &'a str,
    background: &'a str,
    markup: &'a str,
}

impl<'a> Message<'a> {
    fn new(message: String) -> Self {
        Self {
            full_text: format!(" <b>{}</b> ", message.to_uppercase()),
            color: "#111111",
            background: "#eeeeee",
            markup: "pango"
        }
    }
}

impl<'a> Display for Message<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

fn main() {
    let mut session = Session::new().unwrap();
    let mut evt_processor = x11::XkbEventProcessor::new(session.d);

    session.handle_xkb_event(2i32, session.k.get_current_idx(), true);

    loop {
        unsafe {
            match evt_processor.get_next_event() {
                Some(ev) => session.handle_xkb_event(
                    ev.any.xkb_type,
                    ev.state.group.try_into().unwrap(),
                    false
                ),
                None => (),
            }
        }
    }
}

