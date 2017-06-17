macro_rules! syscall    {
    ($sysnum:expr) => {{
        let ret : u64;

        asm!("syscall"
            : "={rax}"(ret)
            : "{rax}"($sysnum as u64)
            : "rcx", "r11", "memory"
            : "intel", "volatile"
        );

        ret
    }};
    ($sysnum:expr, $a1:expr) => {{
        let ret : u64;

        asm!("syscall"
            : "={rax}"(ret)
            : "{rax}"($sysnum as u64), "{rdi}"(($a1) as u64)
            : "rcx", "r11", "memory"
            : "intel", "volatile"
        );

        ret
    }};
}

pub fn getpid() -> ::pid_t  {
    syscall!(0x27) as ::pid_t
}

pub fn getpgid(pid : ::pid_t) -> ::pid_t    {
    let pgid = syscall!(0x79, pid);

    if pgid < 0 {
        panic!("Error in getpgid.")
    } else {
        pgid
    }
}

pub fn getuid() -> ::uid_t  {
    syscall!(0x66) as ::uid_t
}

pub fn geteuid() -> ::uid_t {
    syscall!(0x6b) as ::uid_t
}
