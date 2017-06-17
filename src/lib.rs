#![feature(asm)]
#![allow(non_camel_case_types)]

#![no_std]

pub type pid_t = i32;
pub type uid_t = u32;

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
mod x86_64_apple_darwin;
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub use x86_64_apple_darwin::*;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod x86_64_unknown_linux_gnu;
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub use x86_64_unknown_linux_gnu::*;

mod test    {
    extern  {
        fn getpid() -> ::pid_t;
        fn getpgid(pid : ::pid_t) -> ::pid_t;
        fn getuid() -> ::uid_t;
        fn geteuid() -> ::uid_t;
    }
    
    #[test]
    fn basic_tests()    {
        unsafe  {
            let pid = ::getpid();
            let pid_libc = getpid();
            assert_eq!(pid, pid_libc);
            
            let pgid = ::getpgid(pid_libc);
            let pgid_libc = getpgid(pid_libc);
            assert_eq!(pgid, pgid_libc);
            
            let uid = ::getuid();
            let uid_libc = getuid();
            assert_eq!(uid, uid_libc);
            
            let euid = ::geteuid();
            let euid_libc = geteuid();
            assert_eq!(euid, euid_libc);
        }
    }
}
