#include <inttypes.h>
#include "Print.h"

class LSD : public Print{
  public:
    LSD();
    void render();
    virtual size_t write(uint8_t);

  private:
    static const uint8_t _cols = 16;

    uint8_t _cursor_c = 0;

    uint8_t _char_buffer[_cols];
};

LSD::LSD() {
  for(int i = 0; i < _cols; i++){
    _char_buffer[i] = '_';
  }
}

size_t LSD::write(uint8_t value) {
  _char_buffer[_cursor_c] = value;
  _cursor_c = (_cursor_c + 1) % _cols;
  return 1;
}

void LSD::render() {
  for(int i = 0; i < _cols; i++)
    Serial.print((char) _char_buffer[i]);

  Serial.print("\r");

}

LSD lsd;

void setup() {
  // put your setup code here, to run once:
  pinMode(LED_BUILTIN, OUTPUT);
  Serial.begin(9600);
  lsd.print("hello world");
}

void loop() {
  lsd.render();
  delay(1000);
}
