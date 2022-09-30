# repro_register_class_twice

Reproduction for an [Issue #92](https://github.com/DelSkayn/rquickjs/issues/92) in the https://github.com/DelSkayn/rquickjs Rust wrapper for QuickJS.

It appears that registering a bound struct in multiple contexts of the same runtime is not possible.

```
RUST_BACKTRACE=1 cargo run --bin repro
```

Output:
```
First context
Second context
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: FromJs { from: "null", to: "prototype", message: Some("Tried to get the prototype of class without prototype") }', src/repro.rs:36:42
stack backtrace:
   0: rust_begin_unwind
             at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/panicking.rs:143:14
   2: core::result::unwrap_failed
             at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/result.rs:1785:5
   3: core::result::Result<T,E>::unwrap
             at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/result.rs:1078:23
   4: repro::main::{{closure}}
             at ./src/repro.rs:36:9
   5: rquickjs_core::context::Context::with
             at /home/bspot/.cargo/git/checkouts/rquickjs-c5d9b5f474075f2d/7b2c547/core/src/context.rs:131:26
   6: repro::main
             at ./src/repro.rs:35:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

Possible explanation:

The `Class:register` function (https://github.com/DelSkayn/rquickjs/blob/master/core/src/class.rs#L282) checks whether the class is already registered with the runtime and skips registering it again if that's the case.

However, it also skips registering the prototype (https://github.com/DelSkayn/rquickjs/blob/master/core/src/class.rs#L304).

If I'm not mistaken, this is not correct, since the prototype needs to be associated with the class in each context individually.
