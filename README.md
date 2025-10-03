# The Rules of Crust

<p align=left>
  <img src="./crust.png" width=200>
</p>

1. Use custom build of rustc
1. No references, only pointers.
1. No cargo, build with rustc directly.
1. No std, but libc is allowed.
1. Only Edition 2021.
1. All user structs and enums #[derive(Clone, Copy)].
1. Everything is pub by default.

*The list of rules may change. The goal is to make programming in Rust fun.*

Currently used in the [B Compiler Project](https://github.com/tsoding/b).

## Crust Compiler

1. Unsafe checks removed
1. Variables are mutable by default (`let` rather than `let mut`)
1. Mutable pointers by default (Can use pointers like `*` instead of `*mut` or `**` rather than `*mut *mut`)

> TODO: public without specifying `pub`

## Setup

```bash
git clone git@github.com:mbwilding/rust.git --depth=1
cd rust
./x.py build
```

Then update the path in the `Makefile` in this repo to match your clone location
