#![no_std]
#![no_main]

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
core::arch::global_asm!(
    ".globl __start
__start: and    rsp, 0xfffffffffffffff0 # align stack to 16 bytes; expected by x86-64 Linux C ABI
         call   _start2"
);

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
core::arch::global_asm!(
    ".globl _start
_start: and    rsp, 0xfffffffffffffff0 # align stack to 16 bytes; expected by x86-64 Linux C ABI
        call   start2"
);

#[no_mangle]
#[allow(unused_unsafe)]
unsafe extern "C" fn start2() -> ! {
    let msg = "Hello, World!\n";
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    let syscall_num = 0x02000004;
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    let syscall_num = 1;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") syscall_num,
            in("rdi") 2,
            in("rsi") msg.as_ptr(),
            in("rdx") msg.len(),
            out("rcx") _,
            out("r11") _,
            lateout("rax") _,
        )
    };
    
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    let syscall_num = 0x02000001;
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    let syscall_num = 60;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") syscall_num,
            in("rdi") 0,
            out("rcx") _,
            out("r11") _,
        );
    }
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}