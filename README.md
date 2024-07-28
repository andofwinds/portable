# Portable
### Provides a minimal and safe low-level x86_64 IO port bindings.

This crate is inspired by [x86_64](https://docs.rs/x86_64/)
and designed to be simple, lightweight and pretty to use.

Here is an example reading from `0x01` and writing `0x02` into it:
```rust
// At first, read from port
let foo_port = Port::new(0x01);
let data: u8 = foo_port.read();
                                                                   
// And write
foo_port.write(0x02_u8);
```
*Please, take a note that this is a `no_std` crate and can be used in something like osdev*
