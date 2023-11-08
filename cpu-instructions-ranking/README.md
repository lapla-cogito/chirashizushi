# cpu-instructions-ranking
Tool to display ranking of CPU instruction count appearances

# How to use

```
$ cargo run <target_ELF>
```

or

```
$ cargo run <target_dir>
```

If you enter a ELF file path, you'll see like the following output:

```
Rank 1: mov is executed 817 time(s)
Rank 2: jmp is executed 215 time(s)
Rank 3: lea is executed 179 time(s)
Rank 4: xor is executed 168 time(s)
Rank 5: cmp is executed 161 time(s)
Rank 6: call is executed 150 time(s)
Rank 7: je is executed 122 time(s)
Rank 8: test is executed 112 time(s)
Rank 10: jne is executed 101 time(s)
...
```

If you enter a directory name, this application will analyze all files that can be disassembled by objdump and report the ranking.
