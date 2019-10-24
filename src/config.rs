// These are some values used
// throughout the program

// InControl port to use with aseqdump
// Usually the first one
// Find out the correct port with
//          aseqdump -l
pub const MIDI_PORT_1: &'static str = "24:0";

// InControl port to use with aseqdump
// Usually the second/alternative one
// Find out the correct port with
//          aseqdump -l            
pub const MIDI_PORT_2: &'static str = "24:1";

// InControl port to use with amidi
// Usually the second/alternative one
// Find out the correct port with:
//          amidi --list-devices
pub const MIDI_PORT_3: &'static str = "hw:2,0,1";

// This is the note of the first key
// This can be changed through Octave
pub const FIRST_KEY: usize = 48;

// This is the number of the first drum pad
pub const FIRST_PAD: usize = 112;

// How often an iteration in the 
// scroll check thread happens
// This also controls the scroll speed
// Lower number = More checks
// Lower number = Faster scroll
// This value represents milliseconds
pub const SCROLL_DELAY: u64 = 200;

// How often the resources must be checked
// and how often to update the leds if changed
// Lower number = More checks
// Lower number = More led color updates
// This value represents seconds
pub const LED_DELAY: u64 = 5;