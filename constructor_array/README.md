# constructor_array

Module initialization functions for Rust (like __attribute__((constructor)) in C/C++) under no_std.


After registering a constructor function, a function pointer pointing to it will be stored in the `ctor` section.


When the program starts, it can call all initialization functions in the `ctor` section in order.

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
    constructor_array::invoke_ctors();
    println!(
        "MAX_NUM: {}",
        MAX_NUM.load(std::sync::atomic::Ordering::Relaxed)
    );
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