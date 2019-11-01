use crate::
{
    command_output,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config
{
    // Regular port to use with aseqdump
    // Usually the first one
    // Find out the correct port with
    //          aseqdump -l
    pub midi_port_1: String,

    // InControl port to use with aseqdump
    // Usually the second/alternative one
    // Find out the correct port with
    //          aseqdump -l
    pub midi_port_2: String,

    // Regular port to use with amidi
    // Usually the first one
    // Find out the correct port with:
    //          amidi --list-devices
    pub midi_port_1_b: String,

    // InControl port to use with amidi
    // Usually the second/alternative one
    // Find out the correct port with:
    //          amidi --list-devices
    pub midi_port_2_b: String,

    // This is the note of the first key
    // This can be changed through Octave
    pub first_key: usize,

    // This is the number of the first drum pad
    pub first_pad: usize,

    // How often an iteration in the 
    // scroll check thread happens
    // This also controls the scroll speed
    // Lower number = More checks
    // Lower number = Faster scroll
    // This value represents milliseconds
    pub scroll_delay: u64,

    // How often the resources must be checked
    // and how often to update the leds if changed
    // Lower number = More checks
    // Lower number = More led color updates
    // This value represents milliseconds
    pub led_delay: u64,

    // If this is enabled the program will output
    // some useful information for debugging
    pub debug: bool,
}

pub fn make_config() -> Config
{
    Config
    {
        midi_port_1: command_output("aseqdump -l | grep \"Launchkey MK2 25 MIDI 1\" | awk '{$1=$1};1' \
                        | sed 's/ .*//' | tr -d '\n'"),
        midi_port_2: command_output("aseqdump -l | grep \"Launchkey MK2 25 MIDI 2\" | awk '{$1=$1};1' \
                        | sed 's/ .*//' | tr -d '\n'"),
        midi_port_1_b: command_output("amidi --list-devices | grep \"Launchkey MK2 25 MIDI 1\" \
                        | sed -n '/^IO/s/.*\\(hw[^ ]*\\).*/\\1/p' | tr -d '\n'"),
        midi_port_2_b: command_output("amidi --list-devices | grep \"Launchkey MK2 25 MIDI 2\" \
                        | sed -n '/^IO/s/.*\\(hw[^ ]*\\).*/\\1/p' | tr -d '\n'"),                        
        first_key: 48,
        first_pad: 96,
        scroll_delay: 200,
        led_delay: 5000,
        debug: true,
    }
}