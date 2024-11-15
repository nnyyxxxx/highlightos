#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![warn(clippy::new_without_default)]
#![warn(clippy::missing_safety_doc)]

extern crate alloc;
use alloc::{format, vec};

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

use hlkernel::{
    history::CMD_HISTORY,
    keyboard_buffer, print, println,
    vga_buffer::{Color, WRITER},
};

mod cmd;
use cmd::COMMAND_LIST;

entry_point!(kernel_main);

struct RtrType {
    code: &'static i32,
    info: &'static str,
    color: Color,
}

pub fn init_kernel(boot_info: &'static BootInfo) {
    use hlkernel::allocator;
    use hlkernel::mem::{self, BootInfoFrameAlloc};
    use x86_64::VirtAddr;

    #[cfg(debug_assertions)]
    println!("Initializing...\n");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { mem::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAlloc::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");

    hlkernel::init();

    #[cfg(debug_assertions)]
    WRITER.lock().print_colored(
        format!("\nHighlightOS v{} DEBUG", env!("CARGO_PKG_VERSION")),
        Color::Black,
        Color::Yellow,
    );

    #[cfg(not(debug_assertions))]
    WRITER.lock().print_colored(
        format!("\nHighlightOS v{}", env!("CARGO_PKG_VERSION")),
        Color::Black,
        Color::Yellow,
    );

    print!("\n\nhls < ");
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init_kernel(boot_info);

    loop {
        let input = keyboard_buffer::read_buffer();

        if input.ends_with("\n") {
            keyboard_buffer::clear_buffer();
            CMD_HISTORY.lock().last = 0;

            let mut args: vec::Vec<&str> = input.split(' ').collect();

            if args[0] != "\n" {
                let req_com = &args[0].replace("\n", "");

                if let Some(command) = COMMAND_LIST.iter().find(|&com| com.name == req_com) {
                    args.remove(0);

                    print!("\n");

                    let rtr = (command.fun)(args);

                    if rtr != 1 {
                        if let Some(return_code) = RTR_LIST.iter().find(|&rtr_t| rtr_t.code == &rtr) {
                            println!("\n > {}", req_com);
                            WRITER.lock().print_colored(
                                format!("{} : {}\n\n", rtr, return_code.info),
                                return_code.color,
                                Color::Black, // Color::None ??
                            );
                        } else {
                            println!("\n > {}\nreturned : {}\n", req_com, rtr);
                        }
                    }
                } else {
                    WRITER.lock().print_colored(
                        format!("\n > {}\ncommand not found\n\n", input),
                        Color::LightRed,
                        Color::Black,
                    );
                }

                let mut cmd_history = CMD_HISTORY.lock();
                if !cmd_history.history.is_empty() {
                    if cmd_history.history[cmd_history.history.len() - 1] != input.replace("\n", "") {
                        cmd_history.history.push(input.replace("\n", ""));
                    }
                } else {
                    cmd_history.history.push(input.replace("\n", ""));
                }
            }

            print!("hls < ");
        }
    }
}

const RTR_LIST: &[RtrType] = &[
    RtrType {
        code: &0,
        info: "executed successfully",
        color: Color::Green,
    },
    RtrType {
        code: &2,
        info: "returned general error",
        color: Color::Red,
    },
    RtrType {
        code: &3,
        info: "returned critical error",
        color: Color::Red,
    },
    RtrType {
        code: &4,
        info: "returned user error",
        color: Color::Red,
    },
];

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    WRITER
        .lock()
        .print_colored(format!("KERNEL CRASHED\n{}\n", info), Color::Red, Color::Black);
    hlkernel::hlt_loop();
}
