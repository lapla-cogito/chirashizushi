use regex::Regex;
use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut elf_file_path = "";

    if args.len() > 1 {
        elf_file_path = &args[1];
    }

    if elf_file_path.is_empty() {
        println!("Please input ELF file path");
        return;
    }

    println!("Analyzing {}...", elf_file_path);

    let objdump_output = Command::new("objdump")
        .args(["-d", elf_file_path])
        .output()
        .expect("failed to run objdump");

    let objdump_output_str = String::from_utf8(objdump_output.stdout)
        .expect("failed to convert objdump output to string");

    let mut current_function = "";
    let mut instruction_count = 0;

    for line in objdump_output_str.lines() {
        if let Some(captures) = Regex::new(r"([0-9a-fA-F]+) <([^>]+)>:")
            .unwrap()
            .captures(line)
        {
            let function_name = captures.get(2).unwrap().as_str();
            if !current_function.is_empty() {
                println!(
                    "{} has {} instructions",
                    current_function, instruction_count
                );
            }
            current_function = function_name;
            instruction_count = 0;
        } else if Regex::new(r"^\s+([0-9a-fA-F]+):\s+(.+)$")
            .unwrap()
            .captures(line)
            .is_some()
        {
            instruction_count += 1;
        }
    }

    if !current_function.is_empty() {
        println!(
            "{} has {} instructions",
            current_function, instruction_count
        );
    }
}
