use crate::
{
    s,
    config::*,
    triggers::*,
};

use std::
{
    thread,
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
#[derive(Debug)]
pub struct MidiEvent
{
    pub source: String,
    pub event_1: String,
    pub event_2: String,
    pub channel: String,
    pub label_1: String,
    pub data_1: String,
    pub label_2: String,
    pub data_2: String,
}

fn start_listener(port: &str)
{
    let mut cmd = Command::new("aseqdump")
                    .arg("-p")
                    .arg(port)
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

            let event = MidiEvent
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

            process_midi_event(event);
        }
    }
}

// Start listening to key or slider events
pub fn start_main_listener()
{
    start_listener(MIDI_PORT_1);
}

// Listener for InControl events
pub fn start_ic_listener()
{
    thread::spawn(move || 
    {
        start_listener(MIDI_PORT_2);    
    });
}