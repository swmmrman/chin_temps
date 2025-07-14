
#include <DHT.h>
#include <Adafruit_HDC302x.h>
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
int dht1Pin = 2; // Out
int dht3Pin = 7; // Spare or inside 2.
DHT dht1(dht1Pin, DHT22);  // Out
Adafruit_HDC302x in_sensor = Adafruit_HDC302x();  // In
DHT dht3(dht3Pin, DHT22);  // Spare
LiquidCrystal_I2C lcd(0x27, 20, 4);
int valvePin = 52;
String version = "V1.1.0";

void setup() {
  Serial.begin(115200);
  lcd.init();
  lcd.backlight();
  pinMode(A0, INPUT);
  pinMode(dht1Pin, INPUT);  // Out
  pinMode(dht3Pin, INPUT);  // Spare
  pinMode(valvePin, OUTPUT);
  digitalWrite(valvePin, 0);
  delay(400);
  digitalWrite(valvePin, 1);
  dht1.begin();                 // Out
  in_sensor.begin(0x44, &Wire); // In
  dht3.begin();                 // Spare
  int v = analogRead(A0); 
  for(int i=0; i < numReadings; i++) {
    vals[i] = v;
    total += v;
  }
  maxHumid = sensorMax - 3;
  minHumid = maxHumid - 3;
  //Buzzer Code???
  pinMode(9,OUTPUT);
  TCCR2B = TCCR2B & B11111000 | B00000010;
  analogWrite(9, 128);
  delay(250);
  analogWrite(9,0);
  //
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
    float outTemp = dht1.readTemperature(true);
    double inTemp = 20.55555;
    float spareTemp = dht3.readTemperature(true);
    float outHumid = dht1.readHumidity();
    double inHumid = 4.20;
    float spareHumid = dht3.readHumidity();
    in_sensor.readTemperatureHumidityOnDemand(inTemp, inHumid, TRIGGERMODE_LP0);
    if(timeLeft > 0) {
      timeLeft--;
      if(inHumid > maxHumid) {
        valveOff(false);
      }
    }
    else if(inHumid < minHumid && valveStatus == 0) {
      valveOn();
    }
    else if(valveStatus == 1){
      if(inHumid < maxHumid) {
        valveOff(true);
      }
      else {
        valveOff(false);
      }
    }
    else if(valveStatus == 2) {
      timeOut--;
      if(timeOut <= 0) {
        if(inHumid < maxHumid) {
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
    lcd.print(outTemp);
    lcd.print(F("F,   "));
    lcd.print(outHumid);
    lcd.print(F("%"));
    //Inside line
    lcd.setCursor(0,1);
    lcd.print("In  ");
    lcd.print(inTemp);
    lcd.print(F("F,   "));
    lcd.print(inHumid);
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
    lcd.print(F("In2: "));
    if(spareHumid == 100) {
      lcd.print((String)spareTemp + "f " + (String)spareHumid + "%");
    }
    else {
      lcd.print((String)spareTemp + "f   " + (String)spareHumid + "%");
    }
    
    Serial.print(outTemp);
    Serial.print(F(","));
    Serial.print(inTemp);
    Serial.print(F(","));
    Serial.print(spareTemp);
    Serial.print(F(","));
    Serial.print(outHumid);
    Serial.print(F(","));
    Serial.print(inHumid);
    Serial.print(F(","));
    Serial.print(spareHumid);
    Serial.print(F(","));
    Serial.print(total/numReadings);
    Serial.print(F(","));
    Serial.println(valveStatus);
  }
  delay(200);
}
