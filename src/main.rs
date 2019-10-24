#![allow(clippy::cognitive_complexity)]

mod macros;
mod config;
mod globals;
mod listeners;
mod triggers;
mod leds;

use crate::
{
    config::*,
    listeners::*,
    triggers::*,
    leds::*,
};

use std::
{
    sync::
    {
        Arc,
        atomic::
        {
            AtomicBool, Ordering,
        },
    },
    process::Command,
};

// Program starts here
fn main()
{
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) 
    {
        switch_mode("extended");
        turn_leds_off("both");
        start_led_check();
        start_scroll_check();
        start_ic_listener();
        start_main_listener();
    }

    cleanup();
}

// On termination
fn cleanup()
{
    turn_leds_off("both");
    switch_mode("basic");
}

// Runs a command
fn run_command(cmd: &str)
{
    Command::new("sh").arg("-c").arg(cmd)
        .status().expect("Can't run command.");
}

// Sends a midi signal
fn midi_signal(hex: &str)
{
    run_command(&format!("amidi -p {} -S {}", MIDI_PORT_3, hex));
}

// Sets the controller to extended or basic mode
fn switch_mode(mode: &str)
{
    match mode
    {
        "basic" => midi_signal("9F 0C 00"),
        "extended" => midi_signal("9F 0C 7F"),
        _ => {}
    }
}