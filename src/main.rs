#![allow(clippy::cognitive_complexity)]

mod events;
mod functions;
mod globals;
mod leds;
mod listeners;
mod macros;
mod utils;

use crate::{globals::*, leds::*, listeners::*, utils::*};

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread, time,
};

// Program starts here
fn main() {
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::SIGTERM, Arc::clone(&term)).unwrap();
    signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&term)).unwrap();
    signal_hook::flag::register(signal_hook::SIGQUIT, Arc::clone(&term)).unwrap();

    init_leds();
    start_led_check();
    // start_scroll_check();
    start_ready_countdown();
    start_ic_listener();
    start_main_listener();

    while !term.load(Ordering::Relaxed) {
        sleep(1000);
    }

    cleanup();
}

// On termination
pub fn cleanup() {
    turn_leds_off("both");
    switch_mode("basic");
}

// Sets the program ready after some time
// This is necesarry because using buttons
// while the program is not running
// will add them to a queue that is
// executed right as the program starts
pub fn start_ready_countdown() {
    thread::spawn(move || {
        sleep(g_get_ready_delay());
        g_set_ready(true);
        p!("Ready")
    });
}

// Pause executions
// Time in milliseconds
pub fn sleep(m: usize) {
    thread::sleep(time::Duration::from_millis(m as u64));
}