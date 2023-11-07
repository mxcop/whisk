## The `[profile.*]` section

This section can occur multiple times with different suffixes.<br>
Each different section defines a different profile.

The official profiles are `dev` and `release`.<br>
To build or run in release you can use the `-r --release` flag.

```toml
[package]
name     = "pkg"
compiler = "gcc"
src      = [ "main.c" ]

[profile.dev]
opt-level = "debug"
debug = "full"

[profile.release]
opt-level = "O3"
debug = "none"
```

### `opt-level` Optimization level

The level of optimization used when building the project using this profile.<br>
Options you can choose from:

#### `none` or `O0` or `0`
Most optimizations are completely disabled.

#### `debug` or `Og` or `g`
Optimize debugging experience. (used for dev)

#### `level-1` or `O1` or `1`
Optimize. Optimizing compilation takes somewhat more time, and a lot more memory for a large function.

#### `level-2` or `O2` or `2`
Optimize even more. GCC performs nearly all supported optimizations that do not involve a space-speed tradeoff.

#### `level-3` or `O3` or `3`
Optimize yet more. (used for release)

#### `size` or `Os` or `s`
Optimize for size. -Os enables all -O2 optimizations except those that often increase code size.

#### `size-aggressive` or `Oz` or `z`
Optimize aggressively for size rather than speed.

#### `fast` or `Ofast``
Optimize for speed, disregard strict standards compliance.

### `debug` Debug information level

The amount of debug information included when building the project using this profile.<br>
Options you can choose from:

#### `none` or `0`
Produces no debug information. (used for release)

#### `minimal` or `min` or `1`
Produces minimal information, enough for making backtraces<br>
in parts of the program that you don’t plan to debug.<br>
This includes descriptions of functions and external variables,<br>
and line number tables, but no information about local variables.

#### `full` or `extra` or `3`
Produces extra information, such as all the macro definitions present in the program.<br>
Some debuggers support macro expansion when you use -g3. (used for dev)
