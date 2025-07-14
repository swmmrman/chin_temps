
#include <DHT.h>
#include <LiquidCrystal_I2C.h>

int numReadings = 25;
float sensorMax = 100;
int vals[25];
int counter = 0;
int total = 0;
int timeLeft = 0; //seconds
int runTime = 5; //Add 1 second to desired time
int timeOut = 0;
int waitTime = 11;
int valveStatus = 0;
float minHumid;
float maxHumid;
int dht1Pin = 2;
int dht2Pin = 6;
int dht3Pin = 7;
DHT dht1(dht1Pin, DHT22);
DHT dht2(dht2Pin, DHT22);
DHT dht3(dht3Pin, DHT22);
LiquidCrystal_I2C lcd(0x27, 20, 4);
int valvePin = 52;
String version = "V1.0.0";

void setup() {
  Serial.begin(115200);
  lcd.init();
  lcd.backlight();
  pinMode(A0, INPUT);
  pinMode(dht1Pin, INPUT);
  pinMode(dht2Pin, INPUT);
  pinMode(dht3Pin, INPUT);
  pinMode(valvePin, OUTPUT);
  digitalWrite(valvePin, 0);
  delay(400);
  digitalWrite(valvePin, 1);
  dht1.begin();
  dht2.begin();
  dht3.begin();
  int v = analogRead(A0); 
  for(int i=0; i < numReadings; i++) {
    vals[i] = v;
    total += v;
  }
  maxHumid = sensorMax - 3;
  minHumid = maxHumid - 3;
  pinMode(9,OUTPUT);
  TCCR2B = TCCR2B & B11111000 | B00000010;
  analogWrite(9, 128);
  delay(250);
  analogWrite(9,0);
}

void valveOff(bool wait) {
  if(wait) {
    valveStatus = 2;
    timeOut = waitTime;
  }
  else {
    valveStatus = 0;
  }
  timeLeft = 0;
  digitalWrite(valvePin, 1);
}

void valveOn() {
  valveStatus = 1;
  timeLeft = runTime;
  digitalWrite(valvePin, 0);
}

void loop() {
  int oldV = vals[counter];
  int curV = analogRead(A0);
  vals[counter] = curV;
  total = total - oldV + curV;
  counter = (counter + 1) % numReadings;
  if(counter % 5 == 0){
    float d1Temp = dht1.readTemperature(true);
    float d2Temp = dht2.readTemperature(true);
    float d3Temp = dht3.readTemperature(true);
    float d1Humid = dht1.readHumidity();
    float d2Humid = dht2.readHumidity();
    float d3Humid = dht3.readHumidity();
    if(timeLeft > 0) {
      timeLeft--;
      if(d2Humid > maxHumid) {
        valveOff(false);
      }
    }
    else if(d2Humid < minHumid && valveStatus == 0) {
      valveOn();
    }
    else if(valveStatus == 1){
      if(d2Humid < maxHumid) {
        valveOff(true);
      }
      else {
        valveOff(false);
      }
    }
    else if(valveStatus == 2) {
      timeOut--;
      if(timeOut <= 0) {
        if(d2Humid < maxHumid) {
          valveOn();
        }
        else {
          valveOff(false);
        }
      }
    }
    //lcd.clear();
    //outside info
    lcd.setCursor(0,0);
    lcd.print("Out ");
    lcd.print(d1Temp);
    lcd.print(F("F,   "));
    lcd.print(d1Humid);
    lcd.print(F("%"));
    //Inside line
    lcd.setCursor(0,1);
    lcd.print("In  ");
    lcd.print(d2Temp);
    lcd.print(F("F,   "));
    lcd.print(d2Humid);
    lcd.print(F("%"));
    //Vavle Status
    lcd.setCursor(0,2);
    lcd.print(F("Water is:"));
    if(valveStatus == 0) {
      lcd.print(F(" Off     "));
    }
    else if(valveStatus == 2) {
      lcd.print(F(" Sensing "));
    }
    else {
      lcd.print(F(" Spraying"));
    }
    lcd.setCursor(0,3);
    lcd.print(F("DHT3:"));
    if(d3Humid == 100) {
      lcd.print((String)d3Temp + "f " + (String)d3Humid + "%");
    }
    else {
      lcd.print((String)d3Temp + "f   " + (String)d3Humid + "%");
    }
    
    Serial.print(d1Temp);
    Serial.print(F(","));
    Serial.print(d2Temp);
    Serial.print(F(","));
    Serial.print(d3Temp);
    Serial.print(F(","));
    Serial.print(d1Humid);
    Serial.print(F(","));
    Serial.print(d2Humid);
    Serial.print(F(","));
    Serial.print(d3Humid);
    Serial.print(F(","));
    Serial.print(total/numReadings);
    Serial.print(F(","));
    Serial.println(valveStatus);
  }
  delay(200);
}
