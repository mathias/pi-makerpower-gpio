# pi-makerpower-gpio

Targets `aarch64-unknown-linux-gnu` architecture, uses the GPIO/I2C on the Raspberry Pi Zero 2W to show status of the [MakerPower MPPT](https://github.com/danjulio/MPPT-Solar-Charger) board.

Currently just outputs to STDOUT.

## Compilation

Cross-compile from a PC/Mac with [cross](https://github.com/cross-rs/cross) with the command:

```
cross build --target aarch64-unknown-linux-gnu
```

and copy the file in `target/aarch64-unknown-linux-gnu/debug` called `pi-makerpower-gpio` to your Raspberry Pi Zero 2W.

## TODO:
- [ ] Read "alert" GPIO pin to trigger system shut down
- [ ] Output to file so webserver can render values of charging, battery voltage, figure out charge level.
- [ ] Other types of output?

## Depends on

https://github.com/golemparts/rppal

## License

Copyright (c) 2023 Matt Gauger

MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
