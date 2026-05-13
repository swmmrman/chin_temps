#!/bin/bash

arduino-cli compile -b arduino:avr:mega ldrTests.ino
arduino-cli upload -b arduino:avr:mega -p /dev/ttyUSB0 ldrTests.ino
