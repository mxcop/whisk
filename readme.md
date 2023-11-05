<img src=".github/whisk-banner.png" />

# Whisk<sub>*(ey)*</sub> ~ build system
A simplistic build system for `C` and `C++` projects inspired by `cargo`.
> **Warning**
> `whisk` is already quite functional, nevertheless it is early in development, so expect issues along the way.

## Installing
*wip*: atm the only way to install `whisk` is by cloning this repo and running `cargo install --path .`

## User guide
Whisk projects are defined by a single manifest file called `whisk.toml`.<br>
In snippet `[1]` you can see an example of a manifest file for a simple `C++` project.

<sub>*snippet* `[1]`</sub>

```toml
[package]
name = "cpp-example"
lang = "c++"
type = "exe"                # <optional> Type of project (default "exe")

compiler = "g++"            # Compiler to use
src = [ "src/**/*.cpp" ]    # List of source files
include = [ "inc" ]         # List of include directories
lib = [ "glfw" ]            # List of libraries

[target.x86_64-windows-64]
libs = [ "lib/win64" ]      # Library search directories

[target.x86_64-windows-32]
libs = [ "lib/win32" ]      # Library search directories
```

> More detailed information about the manifest file can be found [here](./docs/manifest.md).

### Commands
Common commands you'll be using may include :
* `whisk build <path> <target> [-v]` builds a project.
* `whisk run <path> [-v]` builds and then runs a project.
* `whisk clean <path>` cleans a project. (*removes /bin directory*)

For more detailed information use `--help` or `-h`.

<br>

<h2></h2>
<div align="right"><sub>© 2023 Max &lt;mxcop&gt;, All rights reserved — <a href="./license.md">MIT</a>.</sub></div>
