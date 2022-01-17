#![allow(clippy::cognitive_complexity)]

mod config;
mod events;
mod functions;
mod globals;
mod leds;
mod listeners;
mod macros;
mod utils;

use crate::{globals::*, leds::*, listeners::*};

use std::{
    process::Command,
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

    switch_mode("extended");
    turn_leds_off("both");
    g_set_leds_ready(true);
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

// Runs a command
pub fn run_command(cmd: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status()
        .expect("Can't run command.");
}

pub fn spawn_command(cmd: &str) {
    Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .spawn()
        .expect("Can't spawn command.");
}

pub fn command_output(cmd: &str) -> String {
    let o = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Can't get command output.");

    String::from_utf8_lossy(&o.stdout).to_string()
}

// Sends a midi signal
pub fn midi_signal(hex: &str) {
    run_command(&format!("amidi -p {} -S {}", g_get_midi_port_2_b(), hex));
}

// Sets the controller to extended or basic mode
pub fn switch_mode(mode: &str) {
    match mode {
        "basic" => midi_signal("9F 0C 00"),
        "extended" => midi_signal("9F 0C 7F"),
        _ => {}
    }
}

// Function used for debugging information
pub fn debug(s: &str) {
    if g_get_debug() {
        p!(s);
    }
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
