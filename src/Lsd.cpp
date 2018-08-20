#include "Lsd.h"
#include "Arduino.h"
#include <inttypes.h>


//default display has one row and 16 cols
//to mimic LiquidCrystal library behavior
Lsd::Lsd() {
  _cols = 16;
  _rows = 1;
}

void Lsd::_end_row() {
  Serial.flush();
}

void Lsd::begin(uint8_t cols, uint8_t rows) {
  _cols = (cols < _max_cols) ? cols : _max_cols;
  _rows = (rows < _max_rows) ? rows : _max_rows;
}

size_t Lsd::write(uint8_t val) {
  Serial.write(CMD_WRITE);
  Serial.write(val);
  Serial.write(0); //padding
  Lsd::_end_row();
  return 1;
}

void Lsd::setCursor(uint8_t col, uint8_t row) {
  Serial.write(CMD_SETC);
  Serial.write(col);
  Serial.write(row);
  Lsd::_end_row();
}

void Lsd::home() {
  Serial.write(CMD_HOME);
  Serial.write(0); // padding
  Serial.write(0);
  Lsd::_end_row();
}

void Lsd::clear(){
  Serial.write(CMD_CLEAR);
  Serial.write(0); // padding
  Serial.write(0);
  Lsd::_end_row();
}