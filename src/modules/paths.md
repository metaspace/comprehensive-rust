# Paths

Paths are resolved as follows:

1. As a relative path:
   * `foo` or `self::foo` refers to `foo` in the current module,
   * `super::foo` refers to `foo` in the parent module.

2. As an absolute path:
   * `crate::foo` refers to `foo` in the root of the current crate,
   * `::bar::foo` refers to `foo` in the `bar` crate.
   * `bar::foo` is equivalent to `self::bar::foo` and refers to `foo` in whatever `bar` resolves to in the current scope.
   * Local names shadow crate names.


A module can bring symbols from another module into scope with `use`.
You will typically see something like this at the top of each module:

```rust,editable
use std::collections::HashSet;
use std::mem::transmute;
```

Use fully qualified paths to resolve name clashes:

```rust,editable
mod rand {
    pub fn do_something() {
        println!("In the rand module");
    }
}

fn main() {
    rand::do_something();
    self::rand::do_something();
    crate::rand::do_something();
    let x: bool = ::rand::random();
    println!("{x}");
}
```

<details>

 * Try to add `use rand` to the top of the example and look at the error.

</details>
