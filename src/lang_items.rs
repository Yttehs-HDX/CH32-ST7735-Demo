use ch32_hal::println;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("panicked, info = {}\r", info);
    loop {}
}
