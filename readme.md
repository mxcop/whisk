# Whisk<sub>*(ey)*</sub> ~ build system
A simplistic build system for `C` and `C++` projects inspired by `cargo` and `npm`.

## User guide
Whisk projects are defined by a single manifest file called `whisk.toml`.<br>
In snippet `[1]` you can see an example of a manifest file for a simple `C++` project.

<sub>*snippet* `[1]`</sub>
```toml
[package]
name = "cpp-example"

[profile]
compiler = "g++"
src = [ "src/**/*.cpp" ]
include = [ "inc" ]
```