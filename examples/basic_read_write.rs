//! NOTE: This is just a dummy code snippet and it cannot be runned as a real case.

use portable::Port;

fn main() {
    // Create a new port.
    let port = Port::new(0_16);

    // Lets read two bytes from it.
    let _read_dat: u16 = port.read();

    // Read to buffer.
    let mut buf: u16 = 0;
    port.read_to(&mut buf);

    // And write into it.
    port.write(11_u16);
}
