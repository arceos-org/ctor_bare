# constructor_array

[![Crates.io](https://img.shields.io/crates/v/constructor_array)](https://crates.io/crates/constructor_array)
[![Docs.rs](https://docs.rs/constructor_array/badge.svg)](https://docs.rs/constructor_array)
[![CI](https://github.com/arceos-org/constructor_array/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/arceos-org/constructor_array/actions/workflows/ci.yml)


Module initialization functions for Rust (like __attribute__((constructor)) in C/C++) under no_std.


After registering a constructor function, a function pointer pointing to it will be stored in the `.init_array` section.


It can support Linux, Windows, MacOS and other systems, and can be also used in `no_std` environments when developing your own kernel.


In Linux, Windows, MacOS and other systems, the `.init_array` section is a default section to store initialization functions. When the program starts, the system will call all functions in the `.init_array` section in order.


When you are running your own operating system, you can call `constructor_array::invoke_ctors` to invoke all registered constructor functions.

## Usage

```rust
use constructor_array::register_ctor;
#[register_ctor]
fn hello_world() {
    println!("Hello, world!");
}

static MAX_NUM: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[register_ctor]
fn set_max_num() {
    MAX_NUM.store(20, std::sync::atomic::Ordering::Relaxed);
}

fn main() {
    assert_eq!(MAX_NUM.load(std::sync::atomic::Ordering::Relaxed), 20);
}
```

Because the `.init_array` section is a default section to store initialization functions in Linux and some other systems, it will be included in the linker script of compilers like GCC and Clang.


**However**, if you are using a custom linker script, you need to **add the `.init_array` section to the `.text` section manually**, so that these functions can be mapped into the page table and executed correctly. You can add the following line to your linker script as a reference:

```test, ignore
.text : ALIGN(4K) {
    # other sections in the `.text` section

    _init_array_start = .;
    _init_array_end = _init_array_start + SIZEOF(.init_array);
    *(.init_array .init_array.*)
    . = _init_array_end;

    # other sections in the `.text` section
}
```

## Notes 
To avoid section-related symbols being optimized by the compiler, you need to add "-z nostart-stop-gc" to the compile flags (see <https://lld.llvm.org/ELF/start-stop-gc>).


For example, in `.cargo/config.toml`:
```toml
[build]
rustflags = ["-C", "link-arg=-z", "link-arg=nostart-stop-gc"]
rustdocflags = ["-C", "link-arg=-z", "-C", "link-arg=nostart-stop-gc"]
```