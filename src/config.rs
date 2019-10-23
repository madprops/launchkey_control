// These are some values used
// throughout the program

// This is the note of the first key
// This can be changed through Octave
pub const FIRST_KEY: usize = 48;

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