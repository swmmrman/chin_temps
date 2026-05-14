
#include <DHT.h>
#include <Adafruit_HDC302x.h>
#include <LiquidCrystal_I2C.h>

String version = "V1.2.0";

int call = 1;
int counter = 0;
int dht1Pin = 2; // Out
int dht2Pin = 3; //
int dht3Pin = 7; // Spare or inside 2.
int numReadings = 25; // Number of LDR readings. vals[]
int runTime = 5; // Spray time. Add 1 second to desired time
// float sensorMax = 100;
int total = 0;
int timeLeft = 0; // Time left for spraying in seconds
int timeOut = 0; // Delay time remaining for sensing.
int vals[25];
int valveStatus = 0;
int valvePin = 8;
int waitTime = 11; // Length of delay for sensing.

double highLimit = 90.0;
double lowLimit = 85.0;


DHT dht1(dht1Pin, DHT22);  // Out
DHT dht2(dht2Pin, DHT22);  // In
DHT dht3(dht3Pin, DHT22);  // Spare
LiquidCrystal_I2C lcd(0x27, 20, 4);

void setup() {
  Serial.begin(115200);
  lcd.init();
  lcd.backlight();
  pinMode(A0, INPUT);
  pinMode(dht1Pin, INPUT);  // Out
  pinMode(dht2Pin, INPUT);
  pinMode(dht3Pin, INPUT);  // Spare
  pinMode(valvePin, OUTPUT);
  digitalWrite(valvePin, 1);
  delay(400);
  digitalWrite(valvePin, 0);
  dht1.begin();             // Out
  dht2.begin();             // In
  dht3.begin();             // Spare
  int v = analogRead(A0);
  for(int i=0; i < numReadings; i++) {
    vals[i] = v;
    total += v;
  }
}

//Call with wait to true for sense time.
//False simply clears the timer and shuts off the vavle.
void valveOff(bool wait) {
  if(wait) {
    valveStatus = 2;
    timeOut = waitTime;
  }
  else {
    valveStatus = 0;
  }
  timeLeft = 0;
  digitalWrite(valvePin, 0);
}

//Converts to F.
double CToF(double temp) {
  return(((9.0/5.0) * temp) + 32);
}

//Turn the vavle and and sets timeleft.
//Valve status of 1 is on, 2 sense, 0, off.
void valveOn() {
  valveStatus = 1;
  timeLeft = runTime;
  digitalWrite(valvePin, 1);
}
void pad(float temp) {
  if(temp <100){
    lcd.print(" ");
  }
  if(temp < 10) {
    lcd.print(" ");
  }
}

void loop() {
  String input = "";
  bool hitNewLine = false;
  if(Serial.available()) {
    while(hitNewLine == false){
      char inByte = (char)Serial.read();
      if(inByte == '\n') {
        hitNewLine = true;
        String command = input.substring(0,1);
        double offset = (double)input.substring(2).toFloat();
        int call_request = (int)input.substring(2).toInt();
        if(command == "H") {
          highLimit += offset;
        }
        else if(command == "L") {
          lowLimit += offset;
        }
        else if(command == "C") {
          call = call_request;
        }
      }
      else {
        input += inByte;
      }
    }
  }
  int oldV = vals[counter];
  int curV = analogRead(A0);
  vals[counter] = curV;
  total = total - oldV + curV;
  counter = (counter + 1) % numReadings;
  if(counter % 5 == 0){
    float outTemp = dht1.readTemperature(true);
    float inTemp = dht2.readTemperature(true);
    float spareTemp = dht3.readTemperature(true);
    float outHumid = dht1.readHumidity();
    float inHumid = dht2.readHumidity();
    float spareHumid = dht3.readHumidity();
    //Check for outside temp is under 64 and shut off or do nothing.
    if(outTemp < 63 || call == 0 ) {
      if( valveStatus != 0) {
        valveOff(false);
      }
    }
    // Check if timeLeft is not zero,(Spraying)
    else if(timeLeft > 0) {
      timeLeft--;
      if(inHumid > highLimit) {
        valveOff(false);
      }
    }
    //Valve is off and humidity inside has drop below threshold.
    else if(inHumid < lowLimit && valveStatus == 0) {
      valveOn();
    }
    //Currently spaying, Check humidity.  Switch to sense
    //Or off if humidity has risen high enouhg.
    else if(valveStatus == 1){
      if(inHumid < highLimit) {
        valveOff(true);
      }
      else {
        valveOff(false);
      }
    }
    //Currently sensing.  Check and retrigger spraying
    //Or clear as needed.
    else if(valveStatus == 2) {
      timeOut--;
      if(timeOut <= 0) {
        if(inHumid < highLimit) {
          valveOn();
        }
        else {
          valveOff(false);
        }
      }
    }
    //outside info
    lcd.setCursor(0,0);
    lcd.print("Out ");
    pad(outTemp);
    lcd.print(outTemp);
    lcd.print(F("F, "));
    pad(outHumid);
    lcd.print(outHumid);
    lcd.print(F("%"));
    //Inside line
    lcd.setCursor(0,1);
    lcd.print("In  ");
    pad(inTemp);
    lcd.print(inTemp);
    lcd.print(F("F, "));
    pad(inHumid);
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
    lcd.print(F("In2:"));
    pad(spareTemp);
    lcd.print(spareTemp);
    lcd.print("F  ");
    pad(spareHumid);
    lcd.print((String)spareHumid + "%");

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
    Serial.print(lowLimit);
    Serial.print(F(","));
    Serial.print(highLimit);
    Serial.print(F(","));
    Serial.print(total/numReadings);
    Serial.print(F(","));
    Serial.print(valveStatus);
    Serial.print(F(","));
    Serial.println(call);
  }
  delay(200);
}
