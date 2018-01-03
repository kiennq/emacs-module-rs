Emacs Module Bindings
====

This crate provides access to the new `Emacs module` functionality recently
introduced in Emacs 25.

Usage aka How to write an oxidized Emacs module in a few easy steps
----
1. Create a new Cargo `lib` project, say `my_fancy_module`
2. Open up `Cargo.toml` in an editor, and:
   * Add `crate-type = ["cdylib"]` to the `[lib]` section
   * Add the following dependencies:
   ```` toml
   libc = "0.2.34"
   emacs = { path = "$EMB_PATH" }
   ````
3. Add the following to your `src/lib.rs`:
   ```` Rust
   extern crate libc;
   #[macro_use]
   extern crate emacs;

   use emacs::{Env, Result, EmacsVal};

   emacs_plugin_is_GPL_compatible!();
   emacs_module_init!(init);

   pub fn init(env: &mut Env) -> Result<EmacsVal> {
       // Add any other things you need the module to do here

       env.provide("my-fancy-module")
   }
   ````
4. Execute `cargo build`
5. If you're on OS X, copy `target/debug/libmy_fancy_module.dylib`
    to `target/debug/libmy_fancy_module.so`
6. Load it in emacs with `(require 'my-fancy-module "/path/to/libmy_fancy_module.so")`.
   Note that this requires Emacs to be configured and compiled with
   the `--with-modules` flag.

For a more elaborate example, check out [test-module](test-module).

Development
----
- Building:
    ````shell
    cargo build --all
    ````
- Testing:
    ````shell
    bin/test.sh
    ````
- Continuous testing (requires `cargo-watch`):
    ````shell
    cargo watch -x 'build --all' -s bin/test.sh
    ````
