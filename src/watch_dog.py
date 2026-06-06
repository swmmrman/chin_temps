#!/usr/bin/python3
import sys

## A simple watch dog script to use the RPi to reset a locked up arduino.
if __name__ != "__main__":
    print("Not a library,  Do not import")
    sys.exit(1)

reset_pin = 26


def reset_arduino():
    return
