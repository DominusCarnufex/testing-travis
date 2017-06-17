macro_rules! syscall    {
    ($sysnum:expr) => {{
        let ret : u64;
        let cfl : u8;

        asm!("syscall\n\t\
              setc dil"
            : "={rax}"(ret), "={dil}"(cfl)
            : "{rax}"($sysnum as u64)
            : "rcx", "rdx", "r11", "memory"
            : "intel", "volatile"
        );

        (ret, cfl)
    }};
    ($sysnum:expr, $a1:expr) => {{
        let ret : u64;
        let cfl : u8;

        asm!("syscall\n\t\
              setc dil"
            : "={rax}"(ret), "={dil}"(cfl)
            : "{rax}"($sysnum as u64), "{rdi}"(($a1) as u64)
            : "rcx", "rdx", "r11", "memory"
            : "intel", "volatile"
        );

        (ret, cfl)
    }};
}

pub unsafe fn getpid() -> ::pid_t  {
    syscall!(0x2000014).0 as ::pid_t
}

pub unsafe fn getpgid(pid : ::pid_t) -> ::pid_t    {
    let (pgid, err) = syscall!(0x2000097, pid);

    if err != 0 {
        panic!("Error in getpgid.")
    } else {
        pgid as ::pid_t
    }
}

pub unsafe fn getuid() -> ::uid_t  {
    syscall!(0x2000019).0 as ::uid_t
}

pub unsafe fn geteuid() -> ::uid_t {
    syscall!(0x2000018).0 as ::uid_t
}
