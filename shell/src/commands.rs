// Copyleft 🄯 2024  Adam Perkowski

struct Command {
    name: &'static str,
    doc: &'static str,
    fun: fn(&mut sh_i) -> i32,
}

fn clrs() {
    print!("\x1B[2J\x1B[1;1H");

    return 0;
}

fn help() {
    println!(
        "HighlightOS Shell

  List of commands:"
    );

    for cmd in COMMAND_LIST {
        println!(". {}  >>  {}", cmd.name, cmd.doc);
    }

    return 0;
}

fn test() {
    println!("hello. this is a test command. it's life goal is to always return 2.");
    return 2;
}

fn cc() {
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

    return 0;
}

fn exit_hls() {
    print!("are you sure you want to exit? [ y/N ] < ");

    let mut inpt = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut inpt).unwrap();

    if inpt.to_lowercase() == "y\n" {
        println!();
        process::exit(1);
    }
    // return 0
    else {
        return 3;
    }
}

const COMMAND_LIST: &[Command] = &[
    Command {
        name: "clrs",
        doc: "clear screen",
        fun: clrs,
    },
    Command {
        name: "help",
        doc: "show list of commands",
        fun: help,
    },
    Command {
        name: "test",
        doc:  "test :)",
        fun: test,
    },
    Command {
        name: "cc",
        doc: "display copyright info",
        fun: cc,
    },
    Command {
        name: "exit",
        doc: "exit the shell :((",
        fun: exit_hls,
    },
];
