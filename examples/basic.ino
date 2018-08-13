
#include <Lsd.h>

Lsd lsd;

/*
Example based on LiquidCrystal basic example
https://www.arduino.cc/en/Tutorial/HelloWorld?from=Tutorial.LiquidCrystal
*/

void setup() {
  //Lsd library does not initialize
  //Serial interface, so this must
  //be done here
  Serial.begin(9600);
  lsd.begin(16, 2);
  lsd.print("hello world");
}

void loop() {
  // you also need to add render()
  // call to actually send display data
  // trough serial interface
  lsd.render();
  lsd.setCursor(0, 1);
  // print the number of seconds since reset:
  lsd.print(millis() / 1000);
  delay(200);
}
