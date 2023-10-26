# The Manifest Format

<sup>The Whisk manifest is heavily inspired by [Cargo manifest]</sup>

[Cargo manifest]: https://doc.rust-lang.org/cargo/reference/manifest.html

The `whisk.toml` file for each package is called its *manifest*. It is written
in the [TOML] format. It contains metadata that is needed to compile the package.

Every manifest file consists of the following sections:

<sub>`*` means required.</sub>

* [`[package]`](#the-package-section) * --- Defines a package.
  * [`name`](#the-name-field) * --- The name of the package.
  * [`std-version`](#the-rust-version-field) --- The minimal supported C/C++ standard version.
* [`[profile]`](profiles.md) --- Compiler settings and optimizations.

## The `[package]` section

The first section in a `mix.toml` is `[package]`.

```toml
[package]
name = "hello_world" # the name of the package (and binary)
```

The only field required by Whisk is [`name`](#the-name-field).

### The `name` field

The package name is an identifier used to refer to the package. It is used
when listed as a dependency in another package, and as the default name of
inferred lib and bin targets.

The name must use only alphanumeric characters, or `-`, or `_`, and cannot be empty.

Note that [`cargo new`] and [`cargo init`] impose some additional restrictions on
the package name, such as enforcing that it is a valid Rust identifier and not
a keyword. [crates.io] imposes even more restrictions, such as:

- Only ASCII characters are allowed.
- Do not use reserved names.
- Do not use special Windows names such as "nul".
- Use a maximum of 64 characters of length.
