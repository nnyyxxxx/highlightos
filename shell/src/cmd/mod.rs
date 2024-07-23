// Copyleft 🄯 2024  Adam Perkowski

// use std::io;
// use std::io::Write;
// use std::process;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use hlshell::println;
use hlshell::vga_buffer::{Color, STR_COLORS, WRITER};

pub struct Command {
    pub name: &'static str,
    pub args: &'static str,
    pub doc: &'static str,
    pub fun: fn(Vec<&str>) -> i32,
}

fn clrs(_args: Vec<&str>) -> i32 {
    WRITER.lock().clear_screen();
    1
}

fn help(_args: Vec<&str>) -> i32 {
    println!(
        "HighlightOS Shell

  List of commands:"
    );

    for cmd in COMMAND_LIST {
        println!(". {} {}  >>  {}", cmd.name, cmd.args, cmd.doc);
    }

    0
}

fn test(_args: Vec<&str>) -> i32 {
    println!("hello. this is a test command. it's life goal is to always return 2.");
    2
}

fn cc(_args: Vec<&str>) -> i32 {
    println!(
        "Copyright (C) 2024  Adam Perkowski

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see https://www.gnu.org/licenses ."
    );

    0
}

// fn exit_hls(_args: Vec<&str>) -> i32 {
//     print!("are you sure you want to exit? [ y/N ] < ");

//     let mut inpt = String::new();

//     io::stdout().flush().unwrap();
//     io::stdin().read_line(&mut inpt).unwrap();

//     if inpt.to_lowercase() == "y\n" {
//         println!();
//         process::exit(0);
//     }
//     // return 0
//     else {
//         3
//     }
// }

fn document(_args: Vec<&str>) -> i32 {
    if !_args.is_empty() {
        let req_com = &_args[0].replace("\n", "");
        if let Some(command) = COMMAND_LIST.iter().find(|&cmd| cmd.name == req_com) {
            println!("{}  >>  {}", command.name, command.doc);
            0
        } else {
            WRITER.lock().change_color(Color::LightRed, Color::Black);
            println!("Command not found.");
            WRITER.lock().change_color(Color::White, Color::Black);
            3
        }
    } else {
        WRITER.lock().change_color(Color::LightRed, Color::Black);
        println!("No command specified.");
        WRITER.lock().change_color(Color::White, Color::Black);
        4
    }
}

fn chcolor(_args: Vec<&str>) -> i32 {
    if _args.len() == 2 {
        let mut new_colors: vec::Vec<Color> = vec![];

        for arg in _args {
            if let Some(color) = STR_COLORS
                .iter()
                .find(|&col| col.name == arg.replace("\n", ""))
            {
                new_colors.push(color.color);
            } else {
                WRITER.lock().change_color(Color::LightRed, Color::Black);
                println!("Color not found: {}", arg);
                WRITER.lock().change_color(Color::White, Color::Black);
                return 4;
            }
        }
        WRITER.lock().change_color(new_colors[0], new_colors[1]);
        0
    } else {
        WRITER.lock().change_color(Color::LightRed, Color::Black);
        println!("Specify both fg and bg color.\nExample usage: chcolor red white");
        WRITER.lock().change_color(Color::White, Color::Black);
        4
    }
}

#[cfg(debug_assertions)]
fn crasher(_args: Vec<&str>) -> i32 {
    println!("CRASHING...\n\n");
    panic!("Invoked by crasher");
}

pub const COMMAND_LIST: &[Command] = &[
    Command {
        name: "clrs",
        args: "",
        doc: "clear screen",
        fun: clrs,
    },
    Command {
        name: "help",
        args: "",
        doc: "show list of commands",
        fun: help,
    },
    Command {
        name: "test",
        args: "",
        doc: "test :)",
        fun: test,
    },
    Command {
        name: "cc",
        args: "",
        doc: "display copyright info",
        fun: cc,
    },
    // Command {
    //     name: "exit",
    //     args: "",
    //     doc: "exit the shell :((",
    //     fun: exit_hls,
    // },
    Command {
        name: "getdoc",
        args: "[cmd]",
        doc: "display doc of selected command",
        fun: document,
    },
    // Command {
    //     name: "reinit",
    //     args: "",
    //     doc: "re-init the system",
    //     fun: crate::init_kernel,
    // },
    Command {
        name: "chcolor",
        args: "[fg] [bg]",
        doc: "change text color",
        fun: chcolor,
    },
    #[cfg(debug_assertions)]
    Command {
        name: "crash_kernel",
        args: "",
        doc: "DEV | panic! the system",
        fun: crasher,
    },
];
