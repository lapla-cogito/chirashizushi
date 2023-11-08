# cpu-instructions-per-func

Displays number of instructions per function in a ELF file.

# How to use
```
$ cargo run <target_ELF>
```

This will make like following output:

```
Analyzing sample...
_init has 8 instructions
.plt has 7 instructions
__cxa_finalize@plt has 3 instructions
printf@plt has 3 instructions
_start has 15 instructions
deregister_tm_clones has 11 instructions
register_tm_clones has 16 instructions
__do_global_dtors_aux has 17 instructions
frame_dummy has 2 instructions
add has 10 instructions
main has 19 instructions
_fini has 4 instructions
```