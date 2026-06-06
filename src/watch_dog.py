#!/usr/bin/python3
## A simple watch dog script to use the RPi to reset a locked up arduino.
import sys
from time import sleep

import RPi.GPIO as GPIO

if __name__ != "__main__":
    print("Not a library,  Do not import")
    sys.exit(1)

RESET_PIN = 26
reset_file = "/tmp/page/reset_arduino"
log_file = "~/logs/evap/evap_errors.log"


def reset_arduino():
    GPIO.output(RESET_PIN, GPIO.LOW)  ## Low triggers reset
    sleep(1)
    GPIO.output(RESET_PIN, GPIO.HIGH)  ## High activates the reset.


GPIO.setmode(GPIO.BCM)
GPIO.setup(RESET_PIN, GPIO.OUT)

# Set GPIO pin high.  Reset on Arduino is active low
GPIO.output(RESET_PIN, GPIO.HIGH)
