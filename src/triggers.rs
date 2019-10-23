use crate::
{
    s, run_command,
    config::*,
    globals::*,
    leds::*,
};

use std::
{
    thread, time,
    process::
    {
        Command, Stdio,
    },
    io::
    {
        BufReader, BufRead,
    },
};

// Struct that holds data
// about a key/slider event
#[allow(dead_code)]
pub struct TriggerEvent
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

// Execute a function associated with a key
pub fn key_function(s: &str)
{
    // w=white b=black
    // w1 means first white key
    // b4 means fourth black key

    match s
    {
        // White keys
        "w1" => run_command("wmctrl -s 0"),
        "w2" => run_command("wmctrl -s 1"),
        "w3" => run_command("wmctrl -s 2"),
        "w4" => run_command("wmctrl -s 3"),
        "w5" => {},
        "w6" => {},
        "w7" => {},
        "w8" => {},
        "w9" => {},
        "w10" => {},
        "w11" => {},
        "w12" => {},
        "w13" => {},
        "w14" => {},
        "w15" => run_command("xdotool key Super_L+l"),

        // Black keys
        "b1" => run_command("xdotool key Super_L+Ctrl+Left"),
        "b2" => run_command("xdotool key Super_L+Ctrl+Right"),
        "b3" => {},
        "b4" => {},
        "b5" => {},
        "b6" => {},
        "b7" => {},
        "b8" => {},
        "b9" => {},
        "b10" => {},
        _ => {}
    }
}

// Gets the key positon
// i.e note 48 -> w1
pub fn get_key_position(note: String) -> String
{
    let fst = FIRST_KEY;
    let n = note.parse::<usize>().unwrap();

    // White keys
    if n == fst {s!("w1")}
    else if n == fst + 2 {s!("w2")}
    else if n == fst + 4 {s!("w3")}
    else if n == fst + 5 {s!("w4")}
    else if n == fst + 7 {s!("w5")}
    else if n == fst + 9 {s!("w6")}
    else if n == fst + 11 {s!("w7")}
    else if n == fst + 12 {s!("w8")}
    else if n == fst + 14 {s!("w9")}
    else if n == fst + 16 {s!("w10")}
    else if n == fst + 17 {s!("w11")}
    else if n == fst + 19 {s!("w12")}
    else if n == fst + 21 {s!("w13")}
    else if n == fst + 23 {s!("w14")}
    else if n == fst + 24 {s!("w15")}

    // Black keys
    else if n == fst + 1 {s!("b1")}
    else if n == fst + 3 {s!("b2")}
    else if n == fst + 6 {s!("b3")}
    else if n == fst + 8 {s!("b4")}
    else if n == fst + 10 {s!("b5")}
    else if n == fst + 13 {s!("b6")}
    else if n == fst + 15 {s!("b7")}
    else if n == fst + 18 {s!("b8")}
    else if n == fst + 20 {s!("b9")}
    else if n == fst + 22 {s!("b10")}

    else {s!("")}
}

// Start listening to key or slider events
pub fn start_trigger_listener()
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

            if chunks[0] == "Waiting" || chunks[0] == "Source"
            {
                continue;
            }

            let event = TriggerEvent
            {
                source: s!(chunks.get(0).unwrap_or(&s!(""))),
                event_1: s!(chunks.get(1).unwrap_or(&s!(""))),
                event_2: s!(chunks.get(2).unwrap_or(&s!(""))),
                channel: s!(chunks.get(3).unwrap_or(&s!(""))),
                label_1: s!(chunks.get(4).unwrap_or(&s!(""))),
                data_1: s!(chunks.get(5).unwrap_or(&s!(""))),
                label_2: s!(chunks.get(6).unwrap_or(&s!(""))),
                data_2: s!(chunks.get(7).unwrap_or(&s!(""))),
            };

            process_trigger_event(event);
        }
    }
}

// Detect and react to key or slider events
pub fn process_trigger_event(e: TriggerEvent)
{
    match &e.event_1[..]
    {
        "Note" =>
        {
            // Ignore if key is being released
            if e.event_2 == "off"
            {
                return;
            }

            match &e.channel[..]
            {
                // Keys
                "0" => key_function(&get_key_position(e.data_1)),
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
        // Pitch bend
        "Pitch" =>
        {
            let n = e.data_1.parse::<isize>().unwrap();
            let direction = if n > 0 {1} else if n < 0 {2} else {0};
            g_set_scroll_direction(direction);
        },
        // Other controls
        "Control" =>
        {
            match &e.data_1[..]
            {
                // Curved right slider
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

// Thread that starts a check
// to see if the system should scroll
pub fn start_scroll_check()
{
    thread::spawn(move || 
    {   
        loop
        {
            let direction = g_get_scroll_direction();
            if direction == 1 {run_command("xdotool click 4")}
            else if direction == 2 {run_command("xdotool click 5")}

            // Define the check speed here
            thread::sleep(time::Duration::from_millis(SCROLL_DELAY));
        }
    });
}