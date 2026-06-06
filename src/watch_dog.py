#!/usr/bin/python3
## A simple watch dog script to use the RPi to reset a locked up arduino.
import sys
from time import sleep

import RPi.GPIO as GPIO

if __name__ != "__main__":
    print("Not a library,  Do not import")
    sys.exit(1)

reset_pin = 26
reset_file = "/tmp/page/reset_arduino"


def reset_arduino():
    return
