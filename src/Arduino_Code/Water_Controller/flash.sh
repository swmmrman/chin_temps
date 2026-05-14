#!/bin/bash

if [ $# -ne 2 ]; then
    arduino-cli compile -b arduino:avr:mega Water_Controller_DHT22.ino
    arduino-cli upload -b arduino:avr:mega -p /dev/ttyUSB0 Water_Controller_DHT22.ino
else
    arduino-cli compile -b arduino:avr:mega Water_Controller.ino
    arduino-cli upload -b arduino:avr:mega -p /dev/ttyUSB0 Water_Controller.ino
fi
