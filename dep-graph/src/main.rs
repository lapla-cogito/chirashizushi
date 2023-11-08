use regex::Regex;
use std::process::Command;
use std::{env, str};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut elf_file_path = "";

    if args.len() > 1 {
        elf_file_path = &args[1];
    }

    if elf_file_path.is_empty() {
        println!("please input ELF file path");
        return;
    }

    println!("Analyzing {}...", elf_file_path);

    let output = Command::new("objdump")
        .args(["-d", elf_file_path])
        .output()
        .expect("failed to execute objdump");

    let disassembly_output =
        str::from_utf8(&output.stdout).expect("failed to convert objdump output to string");

    let mut call_relations: Vec<(String, String)> = Vec::new();

    let mut current_function = String::new();

    if args.contains(&"--mermaid".to_string()) {
        println!("graph TD;");
        for line in disassembly_output.lines() {
            if line.ends_with(':') {
                let re = Regex::new(r"<(.*?)>").unwrap();
                if let Some(captured) = re.captures(line) {
                    current_function = captured.get(1).unwrap().as_str().to_string();
                    current_function = current_function.splitn(2, '@').next().unwrap().to_string();
                    println!("    {}[{}];", current_function, current_function);
                }
            } else if line.contains("call") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(called_function) = parts.iter().find(|&&x| x.starts_with('<')) {
                    let mut called_function =
                        called_function.trim_matches(|c| c == '<' || c == '>');
                    called_function = called_function.splitn(2, '@').next().unwrap();
                    println!("    {} -->|calls| {};", current_function, called_function);
                }
            }
        }
    } else {
        for line in disassembly_output.lines() {
            if line.ends_with(':') {
                let re = Regex::new(r"<(.*?)>").unwrap();
                if let Some(captured) = re.captures(line) {
                    current_function = captured.get(1).unwrap().as_str().to_string();
                }
            } else if line.contains("call") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(called_function) = parts.iter().find(|&&x| x.starts_with('<')) {
                    call_relations.push((current_function.clone(), called_function.to_string()));
                }
            }
        }

        for (caller, callee) in call_relations {
            println!("Function: {} calls {}", caller, callee);
        }
    }
}
