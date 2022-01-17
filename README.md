My own mappings to control my computer with the Launchkey MK2 25.

As of now, it has mappings to change virtual desktops, change the system's audio volume, lock screen, put computer to sleep, alt tab, scroll, zoom in/out, move windows between monitors, play/pause song, next/previous song.

It also uses the drum pad leds to reflect cpu and ram usage. Functions can also be set for pad taps.

It should be easy enough to modify or extend for your own use.

This script assumes the first (white) key is the note number 48.

If not, you can modify the source, or change it via the Launchkey by pressing both octave buttons at the same time.

More things will be added in time as I think of new functions.

In the image below you can see the Launchkey with the indicator leds. The top row indicates cpu usage, while the bottom row indicates ram usage. The colors depend on the usage percentage.

![](https://i.imgur.com/NtrsZtr.jpg)

Having `aseqdump` and `amidi` installed is required.

Tip: Cover the octave buttons. Put something on top to avoid accidental note offsets.
Tip: Don't change the track offset. This happens when you press both of them at the same time.