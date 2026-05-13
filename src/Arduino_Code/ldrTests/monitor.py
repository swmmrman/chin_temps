#! /usr/bin/python
import serial

s = serial.Serial("/dev/ttyUSB0,", 115200, timeout=1)
try:
    while True:
        ser_in = s.readline().decode("utf-8").strip("\n")
        if ser_in != "":
            print(ser_in)
except KeyboardInterrupt:
    print("exiting")
