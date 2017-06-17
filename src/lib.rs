#![feature(asm)]

#![no_std]

pub type pid_t = i32;
pub type uid_t = u32;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod x86_64_unknown_linux_gnu;
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub use x86_64_unknown_linux_gnu::*;

mod test    {
    extern  {
        fn getpid() -> ::pid_t;
        fn getpid(pid : ::pid_t) -> ::pid_t;
        fn getuid() -> ::uid_t;
        fn geteuid() -> ::uid_t;
    }
    
    #[test]
    fn basic_tests()    {
        let pid = ::getpid();
        let pid_libc = getpid();
        assert_equal!(pid, pid_libc);
        
        let pgid = ::getpgid(pid_libc);
        let pgid_libc = getpgid(pid_libc);
        assert_equal!(pgid, pgid_libc);
        
        let uid = ::getuid();
        let uid_libc = getuid();
        assert_equal!(uid, uid_libc);
        
        let euid = ::geteuid();
        let euid_libc = geteuid();
        assert_equal!(euid, euid_libc);
    }
}
