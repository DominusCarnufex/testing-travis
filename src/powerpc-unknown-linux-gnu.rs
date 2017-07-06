macro_rules! syscall    {
    ($sysnum:expr) => {{
        let ret : u32;
        let err : u32;

        asm!("sc"
            : "={r0}"(err), "={r3}"(ret)
            : "{r0}"($sysnum as u32)
            : "r4", "r5", "r6", "r7", "r8", "r9",
              "r10", "r11", "r10", "cr0", "memory"
            : "volatile"
        );

        (ret, err)
    }};
    ($sysnum:expr, $a1:expr) => {{
        let ret : u32;
        let err : u32;

        asm!("sc"
            : "={r0}"(err), "={r3}"(ret)
            : "{r0}"($sysnum as u32), "{r3}"(($a1) as u32)
            : "r4", "r5", "r6", "r7", "r8", "r9",
              "r10", "r11", "r10", "cr0", "memory"
            : "volatile"
        );

        (ret, err)
    }};
}

pub unsafe fn getpid() -> ::pid_t  {
    let (ret, _) = syscall!(0x14); ret as ::pid_t
}

pub unsafe fn getpgid(pid : ::pid_t) -> ::pid_t    {
    let (ret, err) = syscall!(0x84, pid);

    if err != 0 {
        panic!("Error in getpgid.")
    } else {
        ret as ::pid_t
    }
}

pub unsafe fn getuid() -> ::uid_t  {
    let (ret, _) = syscall!(0x18); ret as ::uid_t
}

pub unsafe fn geteuid() -> ::uid_t {
    let (ret, _) = syscall!(0x31); ret as ::uid_t
}
