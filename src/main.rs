mod macros;
mod globals;

use globals::*;

use std::
{
    process::
    {
        Command, Stdio,
    },
    io::
    {
        BufReader, BufRead,
    },
    sync::
    {
        Arc,
        atomic::
        {
            AtomicBool, Ordering,
        },
    },
    thread, time,
};

// Struct that holds data
// about a key/slider event
#[allow(dead_code)]
struct MidiEvent
{
    source: String,
    event_1: String,
    event_2: String,
    channel: String,
    label_1: String,
    data_1: String,
    label_2: String,
    data_2: String,
}

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
                resource_check();
                thread::sleep(time::Duration::from_secs(5));
            }
        });
    
        start_listener();
    }

    cleanup();
}

// Function that runs on termination
fn cleanup()
{
    turn_leds_off("both");
}

// Start listening to key or slider events
fn start_listener()
{
    let mut cmd = Command::new("aseqdump")
                    .arg("-p")
                    .arg("Launchkey MK2 25")
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();

    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines
        {
            let chunks: Vec<String> = line.unwrap()
                                .split_whitespace()
                                .map(|x| s!(x.replace(",", "").trim()))
                                .collect();

            if chunks[0] == s!("Waiting") || chunks[0] == s!("Source")
            {
                continue;
            }

            let event = MidiEvent
            {
                source: s!(chunks[0]),
                event_1: s!(chunks[1]),
                event_2: s!(chunks[2]),
                channel: s!(chunks[3]),
                label_1: s!(chunks[4]),
                data_1: s!(chunks[5]),
                label_2: s!(chunks[6]),
                data_2: s!(chunks[7]),
            };

            process_event(event);
        }
    }
}

// Runs a command
fn run_command(cmd: &str)
{
    Command::new("sh").arg("-c").arg(cmd)
        .status().expect("Can't run command.");
}

// Detect and react to key or slider events
fn process_event(e: MidiEvent)
{
    match &e.event_1[..]
    {
        "Note" =>
        {
            if e.event_2 == s!("off")
            {
                return;
            }

            match &e.channel[..]
            {
                // Piano keys
                "0" =>
                {
                    match &e.data_1[..]
                    {
                        // First white key
                        "48" => run_command("wmctrl -s 0"),
                        // Second white key
                        "50" => run_command("wmctrl -s 1"),
                        // Third white key
                        "52" => run_command("wmctrl -s 2"),
                        // Fourth white key
                        "53" => run_command("wmctrl -s 3"),
                        // Last white key
                        "72" => run_command("xdotool key Super_L+l"),
                        // First black key
                        "49" => run_command("xdotool key Super_L+Ctrl+Left"),
                        // Second black key
                        "51" => run_command("xdotool key Super_L+Ctrl+Right"),
                        _ => {}
                    }
                },
                // Drum pads
                "9" =>
                {
                    turn_leds_off("both");
                    g_set_cpu_level(0);
                    g_set_ram_level(0);
                },
                _ => {}
            }
        },
        "Control" =>
        {
            match &e.data_1[..]
            {
                // Curved slider
                "1" =>
                {
                    // Change volume
                    let v:f64 = e.data_2.parse::<f64>().unwrap() / 127.0;
            
                    let cmd = format!("for sink in `pacmd list-sinks | grep 'index:' | cut -b12-`\n\
                    do\n\
                        pactl set-sink-volume $sink {:.*}\n\
                    done", 2, v);

                    run_command(&cmd);
                },
                // Linear slider
                "7" => {},
                // Stop button
                "114" => run_command("systemctl suspend"),
                _ => {}
            }
        },
        _ => {}
    }
}

// Light up or turn off a led
fn change_led(n: usize, color: &str)
{
    run_command(&format!("amidi -p hw:2,0,1 -S 9F {} {}", 
        g_get_pad(n), g_get_color(color)));
}

// Turn some or all leds off
fn turn_leds_off(mode: &str)
{
    if mode == "top" || mode == "both"
    {
        change_led_range(1, 8, "off");
    }

    if mode == "bottom" || mode == "both"
    {
        change_led_range(9, 16, "off");
    } 
}

// Change leds from a linear range
fn change_led_range(n1: usize, n2: usize, color: &str)
{
    for x in n1..=n2
    {
        change_led(x, color);
    }
}

// Get the proper led level
fn led_level(p: f32) -> usize
{
    if p >= 90.0 {8}
    else if p >= 80.0 {7}
    else if p >= 70.0 {6}
    else if p >= 60.0 {5}
    else if p >= 50.0 {4}
    else if p >= 40.0 {3}
    else if p >= 30.0 {2}
    else {1}
}

// Get the proper led color
fn led_color<'a>(n: usize) -> &'a str
{
    if n >= 6 {"red"}
    else if n >= 3 {"orange"}
    else {"green"}
}

// Get CPU and RAM info 
// and reflect it with the leds
fn resource_check()
{
    // Calculate and reflect CPU usage

    let cpu = psutil::cpu::cpu_percent(1.0).expect("Can't measure cpu usage.") as f32;
    let level = led_level(cpu);

    if g_get_cpu_level() != level
    {
        turn_leds_off("top");
        change_led_range(1, level, led_color(level));
        g_set_cpu_level(level);
    }

    // Calculate and reflect RAM usage

    let ram = psutil::memory::virtual_memory().expect("Can't measure ram usage.").percent;
    let level = led_level(ram);

    if g_get_ram_level() != level
    {
        turn_leds_off("bottom");
        change_led_range(9, 8 + level, led_color(level));
        g_set_ram_level(level);
    }
}