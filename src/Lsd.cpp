#include "Lsd.h"
#include "Arduino.h"
#include <inttypes.h>


//default display has one row and 16 cols
//to mimic LiquidCrystal library behavior
Lsd::Lsd() {
  _cols = 16;
  _rows = 1;
}

void Lsd::begin(uint8_t cols, uint8_t rows) {
  _cols = (cols < _max_cols) ? cols : _max_cols;
  _rows = (rows < _max_rows) ? rows : _max_rows;

  this->clear();
}

void Lsd::_render() {
  for(int i = 0; i < _rows; i++){
    for(int j = 0; j < _cols; j++){
      Serial.print((char) _char_buffer[i][j]);
    }
    Serial.print("  ");
  }

  Serial.print("\r");
}

size_t Lsd::write(uint8_t value) {
  _char_buffer[_cursor_r][_cursor_c] = value;
  _cursor_c++;

  if(_cursor_c >= _cols){
    _cursor_c = 0;
    _cursor_r++;
    if(_cursor_r >= _rows)
      _cursor_r = 0;
  }

  this->_render();
  return 1;
}

void Lsd::setCursor(uint8_t col, uint8_t row) {
  if(col < _cols)
    _cursor_c = col;
  
  if(row < _rows)
    _cursor_r = row;
}

void Lsd::home() {
  this->setCursor(0, 0);
}

void Lsd::clear(){
  this->home();
  for(int i = 0; i < _rows; i++){
    for(int j = 0; j < _cols; j++){
      _char_buffer[i][j] = (uint8_t) '_';
    }
  }
}