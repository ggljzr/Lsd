# LSD (Work in progess)

Library for simulating 16x2 LCD display via serial port. Its purpose is to replace [LiquidCrystal](https://www.arduino.cc/en/Reference/LiquidCrystal) library in testing/debugging cases where you don't have any suitable LCD available.

## Usage

On Arduino replace LCD instance initialized by LiquidCrystal library with one from Lsd library. You'll also need to start serial interface for communication (baud rate 9600 should work fine):

```c++
#include <LiquidCrystal.h>
#include <Lsd.h>

const int rs = 12, en = 11, d4 = 5, d5 = 4, d6 = 3, d7 = 2;
//LiquidCrystal lcd(rs, en, d4, d5, d6, d7); <- instance using LiquidCrystal;
Lsd lcd; //<- instance using simulator

void setup() { 
  Serial.begin(9600); //dont forget to initialize serial as well 
  lcd.begin(16, 2);
  lcd.print("hello world");
}
```

After Arduino code is uploaded, you can start PC application:

```
# it takes used port and baud rate as arguments
> lsd_app COM3 9600
```

You can download precompiled binary for Windows in release section, or you can compile it with [Cargo](https://doc.rust-lang.org/cargo/):

```
> cd lsd_app
# compiles and runs the binary
> cargo run COM3 9600
```

## Supported functions

* `void begin(uint8_t cols, uint8_t rows)` -- (you can call it but it does nothing for now)
* `void setCursor(uint8_t col, uint8_t row)`
* `void cursor()`
* `void noCursor()`
* `void clear()`
* `void home()`
* `size_t write(uint8_t byte)` -- virtual method for Print class
* methods inherited from Arduino Print class (e. g. `print()`)

More functions hopefully coming soon :-).