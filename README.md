`arduino-shades`
==================
A small program that controls automated window shades 😎. 


## Usage

> **Note** this is default configured for an arduino Nano knockoff using ATmega328P and CH340 chips. See [avr-hal-template](https://github.com/Rahix/avr-hal-template) on how to change for other boards. 

1. Download [ravedude](https://github.com/Rahix/avr-hal/tree/next/ravedude)
2. Plug the arduino board into your usb. 
3. Find the board by running `ls -a /dev/`, it should be `/dev/tty.usbserial-110` or `/dev/tty.usbserial-10`. If using CH340, will be something like `/dev/tty.wchusbserial110` instead. 
4. Tell ravedude where to find your board

```bash
export RAVEDUDE_PORT=/dev/tty.usbserial-110
```

5. cargo run

## Credit
Generated using [avr-hal-template](https://github.com/Rahix/avr-hal-template)

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
