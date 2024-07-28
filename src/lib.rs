#![doc = include_str!("../README.md")]
#![no_std]

use core::{arch::asm, fmt::Display};

#[derive(Clone, Copy, Eq, Debug)]
/// An x86 IO port
pub struct Port {
    /// An x86 IO port address
    addr: u16,
}
impl Port {
    /// Creates a new port instance with given address.
    pub fn new(addr: u16) -> Self {
        Self {
            addr,
        }
    }

    /// Reads a T bytes from port.
    pub fn read<T: PortRw>(&self) -> T {
        T::read(self.addr)
    }

    /// Writes a T-sized `value` to port.
    pub fn write<T: PortRw>(&self, value: T) {
        T::write(self.addr, value);
    }
}
impl PartialEq for Port {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}
impl Display for Port {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.addr)
    }
}
impl Default for Port {
    fn default() -> Self {
        Self {
            addr: 0,
        }
    }
}

/// Provides port read / write
pub trait PortRw {
    /// A read behavior (usually assembly insertion).
    fn read(addr: u16) -> Self;

    /// A write behavior.
    fn write(addr: u16, value: Self);
}
impl PortRw for u8 {
    fn read(addr: u16) -> Self {
        let res: Self;
        unsafe {
            asm!(
                "in al, dx",
                out("al") res,
                in("dx") addr,
                options(nomem, nostack, preserves_flags),
            );
        }

        res
    }

    fn write(addr: u16, value: Self) {
        unsafe {
            asm!(
                "out dx, al",
                in("dx") addr,
                in("al") value,
                options(nomem, nostack, preserves_flags),
            );
        }
    }
}
impl PortRw for u16 {
    fn read(addr: u16) -> Self {
        let res: Self;
        unsafe {
            asm!(
                "in ax, dx",
                out("ax") res,
                in("dx") addr,
                options(nomem, nostack, preserves_flags),
            );
        }

        res
    }

    fn write(addr: u16, value: Self) {
        unsafe {
            asm!(
                "out dx, ax",
                in("dx") addr,
                in("ax") value,
                options(nomem, nostack, preserves_flags),
            );
        }
    }

}
impl PortRw for u32 {
    fn read(addr: u16) -> Self {
        let res: Self;
        unsafe {
            asm!(
                "in eax, dx",
                out("eax") res,
                in("dx") addr,
                options(nomem, nostack, preserves_flags),
            );
        }

        res
    }

    fn write(addr: u16, value: Self) {
        unsafe {
            asm!(
                "out dx, eax",
                in("dx") addr,
                in("eax") value,
                options(nomem, nostack, preserves_flags),
            );
        }
    }
}
