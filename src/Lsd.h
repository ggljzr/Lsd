#ifndef Lsd_h
#define Lsd_h

#include <inttypes.h>
#include "Print.h"

#define CMD_INIT 0
#define CMD_WRITE 1
#define CMD_SETC 2
#define CMD_CLEAR 3
#define CMD_HOME 4
#define CMD_CURSOR 5
#define CMD_NOCURSOR 6
#define CMD_SCROLL_RIGHT 7
#define CMD_SCROLL_LEFT 8

class Lsd : public Print{
  public:
    Lsd();
    void begin(uint8_t cols, uint8_t rows);
    void setCursor(uint8_t col, uint8_t row);
    void clear();
    void home();
    void noCursor();
    void cursor();
    void scrollDisplayRight();
    void scrollDisplayLeft();
    virtual size_t write(uint8_t byte);

  private:
    static const uint8_t _max_rows = 4;
    static const uint8_t _max_cols = 32;

    uint8_t _cols = 16;
    uint8_t _rows = 1;

    static void _end_row();
};

#endif