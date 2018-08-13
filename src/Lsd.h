#ifndef Lsd_h
#define Lsd_h

#include <inttypes.h>
#include "Print.h"

class Lsd : public Print{
  public:
    Lsd();
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

#endif