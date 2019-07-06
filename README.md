# f3demo

A minimal example of some code running on the STM32F3DISCOVERY board.

## Dependencies

To build this you'll need:

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  target. Run:

```
$ rustup target add thumbv7em-none-eabihf
```

## Building
Use OpenOCD to connect to the board
```
$ cd /tmp && openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```
In a different terminal, create the ITM file and start reading it
```
$ cd /tmp && touch itm.txt && itmdump -F -f itm.txt
```
In yet another terminal, build and enter GDB with
```
$ cargo run
```
This will break at the main function, so you need to
```
(gdb) continue
```

## License

[MIT License](LICENSE)
