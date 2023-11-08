use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::Command;

const PREFIXES: [&str; 18] = [
    "lock", "rep", "repe", "repne", "lock", "data16", "data32", "addr16", "addr32", "rex", "cs",
    "ss", "ds", "es", "fs", "gs", "wait", "bnd",
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_path = if args.len() > 1 {
        &args[1]
    } else {
        println!("Please input a directory or file path");
        return;
    };

    let mut instruction_count_map: HashMap<String, usize> = HashMap::new();

    if fs::metadata(target_path).unwrap().is_file() {
        process_file(target_path, &mut instruction_count_map);
    } else if fs::metadata(target_path).unwrap().is_dir() {
        process_directory(target_path, &mut instruction_count_map);
    } else {
        println!(
            "Invalid target: {} is neither a file nor a directory",
            target_path
        );
    }

    let mut ranked_instructions: Vec<(&String, &usize)> = instruction_count_map.iter().collect();
    ranked_instructions.sort_by(|a, b| {
        if b.1.cmp(a.1) == std::cmp::Ordering::Equal {
            a.0.cmp(b.0)
        } else {
            b.1.cmp(a.1)
        }
    });

    let mut rank = 1;
    let mut cur = 0;
    for (_, (instruction, count)) in ranked_instructions.iter().enumerate() {
        if instruction.to_string() != "_" {
            println!(
                "Rank {}: {} is executed {} time(s)",
                rank, instruction, count
            );
        }
        if cur != **count {
            cur = **count;
            rank += 1;
        }
    }
}

fn process_file(elf_file_path: &str, instruction_count_map: &mut HashMap<String, usize>) {
    println!("Analyzing {}...", elf_file_path);

    let objdump_output = Command::new("objdump")
        .args(["-d", elf_file_path])
        .output()
        .expect("failed to run objdump");

    let objdump_output_str = String::from_utf8(objdump_output.stdout)
        .expect("failed to convert objdump output to string");

    for line in objdump_output_str.lines() {
        if let Some(_) = Regex::new(r"^\s+([0-9a-fA-F]+):\s+(.+)$")
            .unwrap()
            .captures(line)
        {
            *instruction_count_map
                .entry(
                    line.split_whitespace()
                        .filter(|element| {
                            !element.ends_with(":")
                                && (u64::from_str_radix(element, 16).is_err() || element.len() > 2)
                                && !PREFIXES.contains(element)
                        })
                        .next()
                        .map(|s| s.to_string())
                        .unwrap_or("_".to_string()),
                )
                .or_insert(0) += 1;
        }
    }
}

fn process_directory(dir_path: &str, instruction_count_map: &mut HashMap<String, usize>) {
    let dir_entries = fs::read_dir(dir_path).expect("Failed to read directory");
    for entry in dir_entries {
        if let Ok(entry) = entry {
            let file_path = entry.path();
            cnt += 1;
            if file_path.is_file() {
                process_file(file_path.to_str().unwrap(), instruction_count_map);
            } else if fs::metadata(file_path.clone()).unwrap().is_dir()
                && fs::read_link(file_path.clone()).is_err()
            {
                process_directory(file_path.to_str().unwrap(), instruction_count_map);
            }
        }
    }
}
