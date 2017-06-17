macro_rules! syscall    {
    ($sysnum:expr) => {{
        let ret : u32;

        asm!("int 0x80"
            : "={eax}"(ret)
            : "{eax}"($sysnum as u32)
            : "memory"
            : "intel", "volatile"
        );

        ret
    }};
    ($sysnum:expr, $a1:expr) => {{
        let ret : u32;

        asm!("int 0x80"
            : "={eax}"(ret)
            : "{eax}"($sysnum as u32), "{ebx}"(($a1) as u32)
            : "memory"
            : "intel", "volatile"
        );

        ret
    }};
}

pub unsafe fn getpid() -> ::pid_t  {
    syscall!(0x14) as ::pid_t
}

pub unsafe fn getpgid(pid : ::pid_t) -> ::pid_t    {
    let pgid = syscall!(0x84, pid) as ::pid_t;

    if pgid < 0 {
        panic!("Error in getpgid.")
    } else {
        pgid
    }
}

pub unsafe fn getuid() -> ::uid_t  {
    syscall!(0xc7) as ::uid_t
}

pub unsafe fn geteuid() -> ::uid_t {
    syscall!(0xc9) as ::uid_t
}
