#!/bin/bash

# This is a script to control a linux system
# using the Launchkey MK2 25 controller

declare -A pads;
pads["1"]="28";
pads["2"]="29";
pads["3"]="2A";
pads["4"]="2B";
pads["5"]="30";
pads["6"]="31";
pads["7"]="32";
pads["8"]="33";
pads["9"]="24";
pads["10"]="25";
pads["11"]="26";
pads["12"]="27";
pads["13"]="2C";
pads["14"]="2D";
pads["15"]="2E";
pads["16"]="2F";

declare -A colors;
colors["red"]="6A";
colors["green"]="4B";
colors["yellow"]="C" ;
colors["off"]="00";

function change_led
{
    $(amidi -p hw:2,0,1 -S "9F ${pads[$1]} ${colors[$2]}");
}

function turn_leds_off
{
    if [[ $1 == top || $1 == both ]]; then
        change_led 1 off;
        change_led 2 off;
        change_led 3 off;
        change_led 4 off;
        change_led 5 off;
        change_led 6 off;
        change_led 7 off;
        change_led 8 off;
    fi

    if [[ $1 == bottom || $1 == both ]]; then
        change_led 9 off;
        change_led 10 off;
        change_led 11 off;
        change_led 12 off;
        change_led 13 off;
        change_led 14 off;
        change_led 15 off;
        change_led 16 off;
    fi
}

cpu_level=0;
ram_level=0;

# Light up the drum pad leds
# according to resource usage
function resource_check
{
    while true; do

        # CPU percentage

        cpu=$(cat <(grep 'cpu ' /proc/stat) <(sleep 1 && grep 'cpu ' /proc/stat) | \
                awk -v RS="" '{print ($13-$2+$15-$4)*100/($13-$2+$15-$4+$16-$5)}');

        if (( $(echo "$cpu > 90.0" | bc -l) )); then
            if [[ $cpu_level != 8 ]]; then
                turn_leds_off top;
                change_led 1 red;
                change_led 2 red;
                change_led 3 red;
                change_led 4 red;
                change_led 5 red;
                change_led 6 red;
                change_led 7 red;
                change_led 8 red;
                cpu_level=8;
            fi
                    
        elif (( $(echo "$cpu > 80.0" | bc -l) )); then
            if [[ $cpu_level != 7 ]]; then
                turn_leds_off top;
                change_led 1 red;
                change_led 2 red;
                change_led 3 red;
                change_led 4 red;
                change_led 5 red;
                change_led 6 red;
                change_led 7 red;
                cpu_level=7;
            fi
                    
        elif (( $(echo "$cpu > 70.0" | bc -l) )); then
            if [[ $cpu_level != 6 ]]; then
                turn_leds_off top;
                change_led 1 red;
                change_led 2 red;
                change_led 3 red;
                change_led 4 red;
                change_led 5 red;
                change_led 6 red;
                cpu_level=6;
            fi

        elif (( $(echo "$cpu > 60.0" | bc -l) )); then
            if [[ $cpu_level != 5 ]]; then
                turn_leds_off top;
                change_led 1 yellow;
                change_led 2 yellow;
                change_led 3 yellow;
                change_led 4 yellow;
                change_led 5 yellow;
                cpu_level=5;
            fi

        elif (( $(echo "$cpu > 50.0" | bc -l) )); then
            if [[ $cpu_level != 4 ]]; then
                turn_leds_off top;
                change_led 1 yellow;
                change_led 2 yellow;
                change_led 3 yellow;
                change_led 4 yellow;
                cpu_level=4;
            fi

        elif (( $(echo "$cpu > 40.0" | bc -l) )); then
            if [[ $cpu_level != 3 ]]; then
                turn_leds_off top;
                change_led 1 yellow;
                change_led 2 yellow;
                change_led 3 yellow;
                cpu_level=3;
            fi

        elif (( $(echo "$cpu > 30.0" | bc -l) )); then
            if [[ $cpu_level != 2 ]]; then
                turn_leds_off top;
                change_led 1 green;
                change_led 2 green;
                cpu_level=2;
            fi
                    
        elif (( $(echo "$cpu > 0" | bc -l) )); then
            if [[ $cpu_level != 1 ]]; then
                turn_leds_off top;
                change_led 1 green;
                cpu_level=1;
            fi

        fi

        # RAM percentage

        mem=$(free | awk '/Mem/{printf("%.2f"), $3/$2*100}');

        if (( $(echo "$mem > 90.0" | bc -l) )); then
            if [[ $ram_level != 8 ]]; then
                turn_leds_off bottom;
                change_led 9 red;
                change_led 10 red;
                change_led 11 red;
                change_led 12 red;
                change_led 13 red;
                change_led 14 red;
                change_led 15 red;
                change_led 16 red;
                ram_level=8;
            fi
                    
        elif (( $(echo "$mem > 80.0" | bc -l) )); then
            if [[ $ram_level != 7 ]]; then
                turn_leds_off bottom;
                change_led 9 red;
                change_led 10 red;
                change_led 11 red;
                change_led 12 red;
                change_led 13 red;
                change_led 14 red;
                change_led 15 red;
                ram_level=7;
            fi
                    
        elif (( $(echo "$mem > 70.0" | bc -l) )); then
            if [[ $ram_level != 6 ]]; then
                turn_leds_off bottom;
                change_led 9 red;
                change_led 10 red;
                change_led 11 red;
                change_led 12 red;
                change_led 13 red;
                change_led 14 red;
                ram_level=6;
            fi

        elif (( $(echo "$mem > 60.0" | bc -l) )); then
            if [[ $ram_level != 5 ]]; then
                turn_leds_off bottom;
                change_led 9 yellow;
                change_led 10 yellow;
                change_led 11 yellow;
                change_led 12 yellow;
                change_led 13 yellow;
                ram_level=5;
            fi

        elif (( $(echo "$mem > 50.0" | bc -l) )); then
            if [[ $ram_level != 4 ]]; then
                turn_leds_off bottom;
                change_led 9 yellow;
                change_led 10 yellow;
                change_led 11 yellow;
                change_led 12 yellow;
                ram_level=4;
            fi

        elif (( $(echo "$mem > 40.0" | bc -l) )); then
            if [[ $ram_level != 3 ]]; then
                turn_leds_off bottom;
                change_led 9 yellow;
                change_led 10 yellow;
                change_led 11 yellow;
                ram_level=3;
            fi

        elif (( $(echo "$mem > 30.0" | bc -l) )); then
            if [[ $ram_level != 2 ]]; then
                turn_leds_off bottom;
                change_led 9 green;
                change_led 10 green;
                ram_level=2;
            fi
                    
        elif (( $(echo "$mem > 0" | bc -l) )); then
            if [[ $ram_level != 1 ]]; then
                turn_leds_off bottom;
                change_led 9 green;
                ram_level=1;
            fi

        fi

        sleep 5;

    done
}

turn_leds_off both;

# If off argument is provided
if [[ $1 == off ]]; then
    exit 1;
fi

resource_check &
aseqdump -p "Launchkey MK2 25" | \

# Listen for input
while IFS=" ," read src ev1 ev2 ch label1 data1 label2 data2 rest; 
do
    if [[ $ev1 == Note ]]; then

        # Ignore when key is released
        if [[ $ev2 == off ]]; then
            continue;
        fi

        # Normal note
        if [[ $ch == 0 ]]; then

            # First white key
            if [[ $data1 == 50 ]]; then
                # Go to first desktop
                wmctrl -s 0;
            fi

            # Second white key
            if [[ $data1 == 52 ]]; then
                # Go to second desktop
                wmctrl -s 1;
            fi

            # Third white key
            if [[ $data1 == 54 ]]; then
                # Go to third desktop
                wmctrl -s 2;
            fi

            # Fourth white key
            if [[ $data1 == 55 ]]; then
                # Go to fourth desktop
                wmctrl -s 3;
            fi

            # First black key
            if [[ $data1 == 51 ]]; then
                # Move to previous desktop
                xdotool key Super_L+Ctrl+Left;
            fi

            # Second black key
            if [[ $data1 == 53 ]]; then
                # Go to next desktop
                xdotool key Super_L+Ctrl+Right;
            fi

        # Pad note
        elif [[ $ch == 9 ]]; then
            :
        fi
    
    elif [[ $ev1 == Control ]]; then

        # Round slider
        if [[ $data1 == 1 ]]; then

            # Volume percentage -> i.e 0.55
            volume=$(echo "scale=2; $data2/127" | bc);
            
            # Apply volume change
            for sink in `pacmd list-sinks | grep 'index:' | cut -b12-`
            do
                pactl set-sink-volume $sink $volume
            done

        # Linear slider
        elif [[ $data1 == 7 ]]; then
            :
        fi

    fi
done