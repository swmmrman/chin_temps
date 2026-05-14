#!/bin/bash

if [ $# -eq 2 ]; then
    echo "Compiling for NANO and DHT22"
    arduino-cli compile -b arduino:avr:nano Water_Controller_DHT22/Water_Controller_DHT22.ino
    arduino-cli upload -b arduino:avr:nano -p /dev/ttyUSB0 Water_Controller_DHT22/Water_Controller_DHT22.ino

else
    echo "Compiling for mega with HDC3022"
    arduino-cli compile -b arduino:avr:mega Water_Controller/Water_Controller.ino
    arduino-cli upload -b arduino:avr:mega -p /dev/ttyUSB0 Water_Controller/Water_Controller.ino
fi
