# Evaporative Cooling control system
This project was initially intended for use just on one chinchills breeding setup.
However due to interest in it, I am reworking the project to make it adaptable and
work on other setups.  It is related to another repo.

## Requirements.
Either DHT a set of DHT sensors for all.
Or an optional interior HDC302x sensor on the cooler output.
HDC302x sensors have become VERY hard to get.

* Arduino-cli or Arudio-ide
* DHT sensors
  * DHT sensor library
  * Adafruit Unified Sensor
* HDC sensors - Hard to find but better.
  * Adafruit HDC302x
* LiquidCrystal I2C
* rustup or rust installed from you package manager.
* Python3 - optional
  * python-pyserial - optional

The arduino portion can be used alone without the need of a controlling computer.
It can run in a "safe" mode, where it just maintains the defaults.

## TODO
* Clean up code.
* Drawings of each component.
* Build of materials.
* Documentation.
