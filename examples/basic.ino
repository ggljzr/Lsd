
#include <Lsd.h>

Lsd lsd;

/*
Example based on LiquidCrystal basic example
https://www.arduino.cc/en/Tutorial/HelloWorld?from=Tutorial.LiquidCrystal
*/

void setup() {
  Serial.begin(9600);
  lsd.begin(16, 2);
  lsd.print("hello world");
}

void loop() {
  lsd.render();
  lsd.setCursor(0, 1);
  // print the number of seconds since reset:
  lsd.print(millis() / 1000);
  delay(200);
}
