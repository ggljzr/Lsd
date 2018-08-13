#include <inttypes.h>
#include "Print.h"

class LSD : public Print{
  public:
    LSD();
    void render();
    void begin(uint8_t cols, uint8_t rows);
    void setCursor(uint8_t col, uint8_t row);
    virtual size_t write(uint8_t);

  private:
    static const uint8_t _max_rows = 4;
    static const uint8_t _max_cols = 32;

    uint8_t _cols = 16;
    uint8_t _rows = 1;

    uint8_t _cursor_c = 0;
    uint8_t _cursor_r = 0;

    uint8_t _char_buffer[_max_rows][_max_cols];
};

//default display has one row and 16 cols
//to mimic LiquidCrystal library behavior
LSD::LSD() {
  _cols = 16;
  _rows = 1;
}

void LSD::begin(uint8_t cols, uint8_t rows) {
  _cols = (cols < _max_cols) ? cols : _max_cols;
  _rows = (rows < _max_rows) ? rows : _max_rows;

  for(int i = 0; i < _rows; i++){
    for(int j = 0; j < _cols; j++){
      _char_buffer[i][j] = (uint8_t) '_';
    }
  }
}

void LSD::render() {
  for(int i = 0; i < _rows; i++){
    for(int j = 0; j < _cols; j++){
      Serial.print((char) _char_buffer[i][j]);
    }
    Serial.print("  ");
  }

  Serial.print("\r");
}

size_t LSD::write(uint8_t value) {
  _char_buffer[_cursor_r][_cursor_c] = value;
  _cursor_c++;

  if(_cursor_c >= _cols){
    _cursor_c = 0;
    _cursor_r++;
    if(_cursor_r >= _rows)
      _cursor_r = 0;
  }

  return 1;
}

void LSD::setCursor(uint8_t col, uint8_t row) {
  _cursor_c = col;
  _cursor_r = row;
}

LSD lsd;

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
