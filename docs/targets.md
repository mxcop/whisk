## The `[target.*]` section

This section can occur multiple times with different suffixes.<br>
Each different section defines a different build target.

```toml
[package] # fields directly on the package will be used as default
name     = "pkg"
compiler = "gcc"
src      = [ "main.c", "src/**/*.c" ]
include  = [ "inc/"  , "lib/glfw/include/" ]
lib      = [ "glfw" ]

[target.x86_64-windows-64]
# this overloads the "libs" field in [package] (if there is one)
libs = [ "lib/win64/" ]

[target.x86_64-windows-32]
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
