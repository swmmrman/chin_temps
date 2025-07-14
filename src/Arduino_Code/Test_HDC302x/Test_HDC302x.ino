#include <Adafruit_HDC302x.h>

Adafruit_HDC302x sense1 = Adafruit_HDC302x();

void setup() {
  Serial.begin(115200);
  if(!sense1.begin(0x44, &Wire)) {
    Serial.println("No sensor");
    while(1) {
      if(!sense1.begin(0x44, &Wire)) {
        break;
      }
    }
  }
}

void loop() {
  
}
