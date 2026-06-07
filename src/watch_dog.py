#!/usr/bin/python3
## A simple watch dog script to use the RPi to reset a locked up arduino.
import datetime
import sys
from time import sleep

import RPi.GPIO as GPIO

if __name__ != "__main__":
    print("Not a library,  Do not import")
    sys.exit(1)

RESET_PIN = 26
reset_file = "/tmp/page/reset_arduino"
log_file = "/home/pi/logs/evap/evap_error.log"


def reset_arduino():
    out = open(log_file, "a")
    ts = datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    out.write(f"{ts} Arduino Stuck.  Resetting.\n")
    out.flush()
    out.close()
    file = open(reset_file, "w")
    file.write("")
    file.flush()
    file.close()
    GPIO.output(RESET_PIN, GPIO.LOW)  ## Low triggers reset
    sleep(1)
    GPIO.output(RESET_PIN, GPIO.HIGH)  ## High activates the reset


GPIO.setmode(GPIO.BCM)
GPIO.setup(RESET_PIN, GPIO.OUT, GPIO.PUD_OFF, GPIO.HIGH)

# Set GPIO pin high.  Reset on Arduino is active low
# GPIO.output(RESET_PIN, GPIO.HIGH)
print("Entering watcher loop")
while True:
    try:
        file = open("/tmp/page/reset_arduino", "r")
        req = file.read()
        if req != "":
            reset_arduino()
        file.close()
        sleep(5)
    except KeyboardInterrupt:
        GPIO.cleanup()
        sys.exit()
