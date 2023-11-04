# The Manifest Format

<sup>The Whisk manifest is heavily inspired by the [Cargo manifest]</sup>

[Cargo manifest]: https://doc.rust-lang.org/cargo/reference/manifest.html
[TOML]: https://toml.io/

The `whisk.toml` file for each package is called its *manifest* file. It is written<br>
in the [TOML] format. It contains metadata that is needed to compile the package.

Every manifest file consists of the following sections:

* [`[package]`](#the-package-section) ~ Defines a package.
  * [`name`](#the-name-field) The name of the package. *(and output binary / lib)*
  * [`std-version`](#the-rust-version-field) The C/C++ standard version used by this package.
* [`[profile]`](profiles.md) ~ Compiler settings and optimizations.

## The `[package]` section

The first section in a `whisk.toml` is the `[package]`.

```toml
[package]
name = "hello"  # the name of the package   (and binary)
lang = "c"      # language of the package   ("c" or "c++")
type = "exe"    # type of the package       ("exe" or "lib") 
```

The only field required by Whisk is [`name`](#the-name-field).

### The `name` field

The package name is an identifier used to refer to the package. It is used<br>
when listed as a dependency in another package, and as the default name of<br>
inferred lib and bin targets.

The name must use only alphanumeric characters, or `-`, or `_`, and cannot be empty.

Note that [`cargo new`] and [`cargo init`] impose some additional restrictions on<br>
the package name, such as enforcing that it is a valid Rust identifier and not<br>
a keyword. [crates.io] imposes even more restrictions, such as:

- Only ASCII characters are allowed.
- Do not use reserved names.
- Do not use special Windows names such as "nul".
- Use a maximum of 64 characters of length.

## The `[target.*]` section

This section can occur multiple times with different suffixes.<br>
Each different section defines a different build target.

```toml
[target] # super target (used as default for all targets)
compiler = "gcc"
src      = [ "main.c", "src/**/*.c" ]
include  = [ "inc/"  , "lib/glfw/include/" ]
lib      = [ "glfw" ]

[target.x86-windows-64]
libs = [ "lib/win64/" ]

[target.x86-windows-32]
libs = [ "lib/win32/" ]
```

Good practice for naming your targets is to use the following format:<br>
[`<target_arch>`] - [`<target_os>`] - [`<target_pointer_width>`]

[`<target_arch>`]: #target_arch-examples
[`<target_os>`]: #target_os-examples
[`<target_pointer_width>`]: #target_pointer_width-examples

### `target_arch` examples
* `"x86"`
* `"x86_64"`
* `"mips"`
* `"powerpc"`
* `"powerpc64"`
* `"arm"`
* `"aarch64"`

### `target_os` examples
* `"windows"`
* `"macos"`
* `"ios"`
* `"linux"`
* `"android"`
* `"freebsd"`
* `"dragonfly"`
* `"openbsd"`
* `"netbsd"`

### `target_pointer_width` examples
* `"8"`
* `"16"`
* `"32"`
* `"64"`
