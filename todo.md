# Todo List

## Features <sup>`prio`</sup>
- [x] Static library target option.
- [x] Checking if manifest file(s) patterns return files.
- [x] Verbose logging flag for debugging. `-v --verbose`
- [x] Manifest file build targets. `[target.*]`
- [ ] Better naming for manifest fields `lib` & `libs`.
- [ ] Precompiled headers.
- [ ] Dynamic library target. (using `-dynamic` flag on `type="lib"` projects)
- [ ] `whisk package` Generates a package folder containing a build of the project with all assets and libs.
- [ ] `prebuild` & `postbuild` fields in manifest to run scripts before and after a build.
- [ ] `xargs` field in manifest for manually specifying compiler arguments.
- [ ] Warning & Error related options in manifest file. (`-Wall`, etc)
- [ ] `whisk new` Creates a new whisk project.

## Features <sup>`nice2have`</sup>
- [ ] `whisk upload` Calls `whisk package` and then runs a script with env variable for the package location.
- [ ] Dependency tree generation and proper diff checking using (`-MM`) option on preprocessor.
- [ ] Custom gcc output parser. (*for errors and warnings*)
