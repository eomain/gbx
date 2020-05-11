# gbx
gbx is a custom (unofficial) toolchain for the Game Boy. It is intended to
enable the development of game software via use of a custom assembly language,
tailored towards the Game Boy hardware.

As of now, gbx is in its early stages and does not have a stable release. Some
features that may be missing are still in development.

**Tools:**

Eventual Components of the toolchain:
- `gb-as`
- `gb-dis`
- `gb-ld`

## Build tools
You can build the entire toolchain using cargo.
```bash
# Build the tools
cargo build --release
```

## Sample program
```asm
.text ; begin text segment

_start: ; start of program
    nop ; no operation
```

## Assemble programs
You can assemble programs with `gb-as`.
```bash
# Assemble `game.s` and output `rom.bin`
gb-as game.s -o rom.bin
```
