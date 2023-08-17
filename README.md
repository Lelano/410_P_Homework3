# breakout: MB2 Rust Breakout Game 
Adam Marion
CS410P Rust Embedded Programming
8/16/2023 Homework 3

Writeup:

Performing the first few tasks of this assignment was relatively straight forward. 
Since the beeping is global scope, I just added a beep to the game.rs on block hit 
and three beeps after the game is complete in main.rs.
Dimming on hit was also relatively straightforward with adding a few lines of code
(game.rs: 134-139) checking if a block is at half life and dimming accordingly. 
The most complicated for me was task three, mainly because I wanted the default 
behavior of the knob to be maintained. The read() function on the knob returns
 and Option<f32> type which does not support the + operator. It either needs 
 to be unwraped() which could be unsafe, or make a checking assignment such as 
 the provided example in (game.rs: 87 - 90) so this made the code solution a bit 
 more type dependent. The other issues is how to determine if the pot was moved. 
 There is noise in the voltage so rarely does the POT k of the previous step 
 match the current step. To mitigate this issue, I had to utilize a range of 
 values to have a threshold of when I would assume the POT was moved and is 
 active (main.rs: 67-75). I also wanted the game to default to the None behavior 
 if the pot is removed. Unfortunatly, when I measured the voltage accrose the POT, 
 Voltage of 0 corresponds to a k of 1.0 and a voltage of 3.3 corresponds to a k 
 of 0, so the POT needs to be connected to get the default None game state and 
 I did not want to modify the code to correct that. Another challenge was that 
 I wanted the transition from POT control to button control to be smooth 
 (no platform jumping around) so I tried using the f32 round function but 
 found that it is not in the core crate! Instead of importing I just decided 
 to forgo rounding the k to the nearest 0.1 on the buttons since it seemed 
 accurate enough. If I have time before Friday I will go ahead and try adding 
 additional sounds and screens, but for now the required scope is complete. 


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
