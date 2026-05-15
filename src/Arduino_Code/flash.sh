#!/bin/bash

elf=$0
print_usage() {
    echo "Usage: $elf board-type sensor-type port"
}

print_help(){
    print_usage
    echo -e "
  Currently supported boards are mega and uno
    Sensor can be DHT22 or HDC for HDC302x
    Port will either be
      /dev/ttyUSBx for some mega boards or
      /dev/ttyACMx for most other arduino
      x is 0 usualy if one adruino is connected.
"
}

port="/dev/ttyUSB0"
if [ $# -gt 1 ]; then
    if [ "$1" = "nano" ]; then
        fqbn="arduino:avr:nano"
        board="Arduino Nano"
    elif [ "$1" = "mega" ]; then
        fqbn="arduino:avr:mega"
        board="Arduino Mega"
    fi
    if [ "$2" = "DHT22" ]; then
        sensor="DHT22"
        target="Water_Controller_DHT22"
    elif [ "$2" = "HDC" ]; then
        sensor="HDC302x"
        target="Water_Controller"
    else
        echo "No valid sensor type."
        exit 1
    fi
    echo "Compiling for $1 with $sensor on $port"
    arduino-cli compile -b $fqbn $target/$target.ino
    arduino-cli upload -b $fqbn -p $port $target/$target.ino
elif [ "$1" = "help" ]; then
    print_help
else
    print_usage
    exit 1
fi
