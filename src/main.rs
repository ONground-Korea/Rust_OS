#![no_std] // Rust 표준 라이브러리를 링크하지 않도록 합니다
#![no_main] // Rust 언어에서 사용하는 실행 시작 지점 (main 함수)을 사용하지 않습니다
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

/// 패닉이 일어날 경우, 이 함수가 호출됩니다.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
