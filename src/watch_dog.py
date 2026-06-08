#!/usr/bin/python3
## A simple watch dog script to use the RPi to reset a locked up arduino.
import datetime
import os
import sys
from pathlib import Path
from time import sleep

import RPi.GPIO as GPIO

if __name__ != "__main__":
    print("Not a library,  Do not import")
    sys.exit(1)

RESET_PIN = 26
RESET_FILE = "/tmp/page/reset_arduino"
LOG_FILE = "/var/logs/evap/watch_dog.log"


def reset_arduino():
    out = open(LOG_FILE, "a")
    ts = datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    out.write(f"{ts} Arduino Stuck.  Resetting.\n")
    out.flush()
    out.close()
    GPIO.output(RESET_PIN, GPIO.LOW)  ## Low triggers reset
    sleep(1)
    GPIO.output(RESET_PIN, GPIO.HIGH)  ## High activates the reset


GPIO.setmode(GPIO.BCM)
GPIO.setup(RESET_PIN, GPIO.OUT, GPIO.PUD_OFF, GPIO.HIGH)

reset_file_preset = os.path.exists(RESET_FILE)
log_file_present = os.path.exists(LOG_FILE)

if not reset_file_preset:
    os.mkfifo(RESET_FILE)

if not log_file_present:
    log_file_handle = open(LOG_FILE, "w")
else:
    log_file_handle = open(LOG_FILE, "a")

print("Entering watcher loop")

reset_file_handle = open(RESET_FILE, "r")

while True:
    try:
        req = reset_file_handle.read()
        if req != "":
            reset_arduino()
        sleep(5)
    except KeyboardInterrupt:
        GPIO.cleanup()
        sys.exit()
