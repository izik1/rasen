# License
This code is licensed under Blue Oak Model License 1.0.0. (see: [license](LICENSE.md))

# Special Thanks
data directly derived from asmdb is used for code generation.

# Usability
This project is _probably_ not in a usable state, however, 
the author has managed to use it in one of their own projects.

### Usability TODO:
* Documentation doesn't exist.
* Breaking changes are likely to be common.
* Missing most variants of most instructions.
* Uses a build script for generating instructions 
  * (to be replaced with a `bin` + commit hook)
  * Currently more bloated than we need to be due to this 
  (can't compile without serde)
* Some instructions _may_ be flat out broken.
  * Tests don't exist for anything except most of the direct encoding functions.
* Missing other target arches.
* Don't use macros where possible. (better IDE completion)
