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
* Some instructions _may_ be flat out broken.
  * Tests don't exist for anything except most of the direct encoding functions.
* Missing other target arches.

## Modifying
After modifying anything to do with codegen, you need to run:
```shell script
./generate
```
This is to allow IDE completion, allow downstream dependents
to not have to depend on serde, and enable faster downstream
compile times of the crate.

`pre-commit` is also supported if you wish to use that.