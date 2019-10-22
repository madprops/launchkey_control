use crate::
{
    s, run_command,
    globals::
    {
        g_set_cpu_level,
        g_set_ram_level,
    },
    leds::
    {
        turn_leds_off,
    },
};

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
                source: s!(chunks[0]),
                event_1: s!(chunks[1]),
                event_2: s!(chunks[2]),
                channel: s!(chunks[3]),
                label_1: s!(chunks[4]),
                data_1: s!(chunks[5]),
                label_2: s!(chunks[6]),
                data_2: s!(chunks[7]),
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