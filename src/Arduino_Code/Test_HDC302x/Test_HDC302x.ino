#include <Adafruit_HDC302x.h>
#include <LiquidCrystal_I2C.h>


Adafruit_HDC302x sense1 = Adafruit_HDC302x();
Adafruit_HDC302x sense2 = Adafruit_HDC302x();
LiquidCrystal_I2C lcd(0x27, 20, 4);

void setup() {
  Serial.begin(115200);
  if(!sense1.begin(0x44, &Wire)) {
    Serial.println("No sensor 1");
    while(1) {
      if(!sense1.begin(0x44, &Wire)) {
        break;
      }
    }
  }
  if(!sense2.begin(0x45, &Wire)) {
    Serial.println("No sensor 2");
  }
}

void loop() {
  double temp = 0.0;
  double RH = 0.0;

  sense1.readTemperatureHumidityOnDemand(temp, RH, TRIGGERMODE_LP0);
  Serial.print("Temp: ");
  Serial.print(ctof(temp));
  Serial.print("f.  RH: ");
  Serial.println(RH);
  delay(1000);
}

double ctof(double c) {
  double f = (c * 1.8) + 32;
  return f;
}
