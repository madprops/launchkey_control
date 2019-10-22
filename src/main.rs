mod macros;
mod globals;
mod triggers;
mod leds;

use crate::
{
    triggers::
    {
        start_trigger_listener,
    },
    leds::
    {
        turn_leds_off,
        update_leds,
    },
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
    thread, time,
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
        turn_leds_off("both");

        thread::spawn(move ||
        {
            loop
            {
                update_leds();
                thread::sleep(time::Duration::from_secs(5));
            }
        });
    
        start_trigger_listener();
    }

    cleanup();
}

// On termination
fn cleanup()
{
    turn_leds_off("both");
}

// Runs a command
fn run_command(cmd: &str)
{
    Command::new("sh").arg("-c").arg(cmd)
        .status().expect("Can't run command.");
}