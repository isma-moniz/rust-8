# rust-8, a rust chip-8 emulator

This is a chip-8 emulator written in Rust.
It supports almost every rom I've tested.

To be fair, calling it an emulator would be a bit of a misnomer. chip-8 is an interpreted programming language which was initially used in the *COSMAC VIP* computer.
The interpreter was stored in positions 0x0 -> 0x200 in memory, and the programs were loaded into memory immediately after that. This emulator mimicks that design by loading the roms into memory from position 0x200 on, reserving the first 0x200 positions for the font and a few other things.


## Keybinds

The following keymap was used:

```
Keypad       Keyboard
+-+-+-+-+    +-+-+-+-+
|1|2|3|C|    |1|2|3|4|
+-+-+-+-+    +-+-+-+-+
|4|5|6|D|    |Q|W|E|R|
+-+-+-+-+ => +-+-+-+-+
|7|8|9|E|    |A|S|D|F|
+-+-+-+-+    +-+-+-+-+
|A|0|B|F|    |Z|X|C|V|
+-+-+-+-+    +-+-+-+-+
```

This keymap is widely used for similar emulators and is the most comfortable in mimicking the OG *COSMAC VIP* layout.

## Running the program

```cargo run <input_rom>```

and enjoy :)
