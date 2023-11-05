# The Manifest Format

<sup>The Whisk manifest is heavily inspired by the [Cargo manifest]</sup>

[Cargo manifest]: https://doc.rust-lang.org/cargo/reference/manifest.html
[TOML]: https://toml.io/

The `whisk.toml` file for each package is called its *manifest* file. It is written<br>
in the [TOML] format. It contains metadata that is needed to compile the package.

Every manifest file consists of the following sections:

* [`[package]`](#the-package-section) ~ Defines a package.
  * [`name`](#the-name-field) The name of the package. *(and output binary / lib)*
  * [`lang`](#the-lang-field) The language this package uses. *(c / c++)*
  * [`type`](#the-type-field) The type of this package. *(exe / lib)*
  * [`...target`](targets.md) All target fields are also available in the [`[package]`](#the-package-section).
* [`[target.*]`](targets.md) ~ Compiler settings and build configuration.

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

### The `lang` field

The package language is used to tell the compiler how to interpret your source and header files.<br>
It can only be one of three options :

- `c` for C projects.
- `c++` for C++ projects.
- `auto` for mixed projects. *(not recommended)*

### The `type` field

The package type dictates what the compiler output should be.<be>
It can only be one of two options :

- `exe` for executables *(programs)*
- `lib` for libraries *(static / dynamic)*
