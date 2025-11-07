# The Rules of Crust

<p align=left>
  <img src="./crust.png" width=200>
</p>

*The list of rules may change. The goal is to make programming in Rust fun.*

Currently used in the [B Compiler Project](https://github.com/tsoding/b).

## 1. No std.

Use [`#![no_std]`][no_std] attribute to enforce this rule. [core][core] is allowed because there is currently [no stable way to disable it](https://github.com/rust-lang/rust/issues/29639). Using libc is allowed since rustc links with it anyway. But since no cargo is allowed you have to declare the necessary libc functions yourself in your crate.

You may also consider enabling [`#![no_main]`][no_main] and provide your custom C-style entry point to be able to get an access to command line arguments.

## 2. Every function is unsafe.

Every single user-made function must be marked as unsafe.

## 3. Raw pointers instead of references.

Raw pointers instead of references must be used for:

1. Parameters and results of all the user-made functions;
2. Members of user-made structs and enums;
3. User-made global variables;

In the bodies of the functions references are allowed to be used for local variables and intermediate value.

## 4. No cargo.

Build with rustc directly pure C-style. Linking with external C libraries is encouraged.

## 5. Only Edition 2021.

Newer additions are too hostile towards Crust.

## 6. Copy by default.

All user-made structs and enums must be `#[derive(Clone, Copy)]`

## 7. Public by default.

Everything is `pub` by default.

[core]: https://doc.rust-lang.org/stable/core/index.html
[no_std]: https://doc.rust-lang.org/reference/names/preludes.html#the-no_std-attribute
[no_main]: https://doc.rust-lang.org/reference/crates-and-source-files.html?highlight=no_main#the-no_main-attribute
