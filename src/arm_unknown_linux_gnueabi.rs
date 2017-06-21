macro_rules! syscall    {
    ($sysnum:expr) => {{
        let ret : u32;

        asm!("svc 0"
            : "={r0}"(ret)
            : "{r7}"($sysnum as u32)
            : "memory"
            : "volatile"
        );

        ret
    }};
    ($sysnum:expr, $a1:expr) => {{
        let ret : u32;

        asm!("svc 0"
            : "={r0}"(ret)
            : "{r7}"($sysnum as u32), "{r0}"(($a1) as u32)
            : "memory"
            : "volatile"
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
    syscall!(0x18) as ::uid_t
}

pub unsafe fn geteuid() -> ::uid_t {
    syscall!(0x31) as ::uid_t
}
