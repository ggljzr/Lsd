#include <inttypes.h>
#include "Print.h"

class LSD{
  public:
    LSD();
    void render();

  private:
    static const uint8_t _cols = 16;

    uint8_t _upper_row[_cols];
    uint8_t _lower_row[_cols];
};

LSD::LSD() {
  for(int i = 0; i < _cols; i++){
    _upper_row[i] = 'u';
    _lower_row[i] = 'l';
  }
}

void LSD::render() {
  for(int i = 0; i < _cols; i++)
    Serial.print((char) _upper_row[i]);

  Serial.print("  ");
  
  for(int i = 0; i < _cols; i++)
    Serial.print((char) _lower_row[i]);

  Serial.print("\r");

}

LSD lsd;

void setup() {
  // put your setup code here, to run once:
  pinMode(LED_BUILTIN, OUTPUT);
  Serial.begin(9600);
}

void loop() {
  lsd.render();
  delay(1000);
}
