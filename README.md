# breakout: MB2 Rust Breakout Game
Bart Massey 2023

This is a demo of
[Breakout](https://en.wikipedia.org/wiki/Breakout_%28video_game%29)
on the [MicroBit v2](https://microbit.org/new-microbit/)
(MB2). Due to the limited display and controls, Breakout is
a challenge on the MB2: this code is a preliminary
demonstration of what might be possible.

## Build and Run

To use this program, you will need to have a potentiometer
(pot) connected to the edge connector of the MB2 to drive
the paddle. The [Adafruit Dragon
Tail](https://www.adafruit.com/product/3695), a breadboard,
and a 100K PCB through-hole pot are one way to get started.
Connect pin 1 of the pot to +3.3V, pin 3 to ground, and pin
2 to P0 (Ring 0) on the MB2 edge connector.

The program can be built with `cargo build --release`. It
can be uploaded with `cargo embed` via `cargo embed
--release`, `probe-run` via `cargo run --release`, using any
CMSIS-DAP connector, or via the MB2 virtual SD card.

# License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
