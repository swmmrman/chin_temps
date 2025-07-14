#include <Adafruit_HDC302x.h>
#include <LiquidCrystal_I2C.h>


Adafruit_HDC302x sense1 = Adafruit_HDC302x();
Adafruit_HDC302x sense2 = Adafruit_HDC302x();
LiquidCrystal_I2C lcd(0x27, 20, 4);
bool sense2_found = true;

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
    sense2_found = false;
    Serial.println("No sensor 2");
  }
  lcd.init();
  lcd.backlight();
}

void loop() {
  double temp1 = 0.0;
  double RH1 = 0.0;
  double temp2 = 20.5555;
  double RH2 = 4.20;

  sense1.readTemperatureHumidityOnDemand(temp1, RH1, TRIGGERMODE_LP0);
  if(sense2_found) {
    sense2.readTemperatureHumidityOnDemand(temp2, RH2, TRIGGERMODE_LP0);
  }
  Serial.print("Temp 1: ");
  Serial.print(ctof(temp1));
  Serial.print("f.  RH 1: ");
  Serial.print(RH1);
  if(sense2_found) {
    Serial.print(" Temp 2: ");
    Serial.print(ctof(temp2));
    Serial.print("f.  RH 2: ");
    Serial.print(RH2);
  }
  lcd.setCursor(0,0);
  lcd.print("Sensor 1:");
  lcd.setCursor(0,1);
  lcd.print(ctof(temp1));
  lcd.print("f");
  lcd.setCursor(0,2);
  lcd.print(RH1);
  lcd.print("%");
  if(sense2_found) {
    lcd.setCursor(10,0);
    lcd.print("Sensor 2:");
    lcd.setCursor(10,1);;
    lcd.print(ctof(temp2));
    lcd.setCursor(10,2);
    lcd.print(RH2);
    lcd.print("%");
  }
  Serial.println("");
  delay(1000);
}

double ctof(double c) {
  double f = (c * 1.8) + 32;
  return f;
}
