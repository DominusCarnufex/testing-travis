#![feature(asm)]

#![no_std]

pub type pid_t = i32;
pub type uid_t = u32;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod x86_64_unknown_linux_gnu;
