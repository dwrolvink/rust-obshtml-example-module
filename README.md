# Introduction
This is an example module using the https://github.com/dwrolvink/obshtml-rust-module-lib crate to create a custom ObsidianHtml module with Rust.

# Compiling
Before you can run this example, you will need to compile the rust code.

To compile the code, first install rust along with cargo:
- https://www.rust-lang.org/tools/install

Then, run:
``` bash
cargo build --release
```

This will compile `./target/release/obshtml-example` (or `./target/release/obshtml-example.exe`, if you are on Windows).

> Note: this crate is not tested on Windows, nor will it be, and it (will) make extensive use of Posix paths. It will probably not work as is on Windows.

# Use with ObsidianHtml
> SECTION OUT OF DATE.
> At time of writing, the python (shim) code described below in this section is not updated. The current goal is to make the shim code superfluous by having ObsidianHtml call the executable directly. Work in progress.

The `target` folder is gitignored by default, and it's easiest to keep it like this.

To make the compiled binary persist, copy it to `obsidianhtml_rust_module_example/src/obshtml-example`.

The python code will execute the binary from that location, so if you skip this step, you will not effectively update the binary after making changes!

On linux you can run `./build`, it combines the two steps above.

## Running
To test your module, you can run:
``` bash
python test.py
```

This will import and instantiate the `ObsidianHtmlRustExampleModule` and run its `run()` method the same way that ObsidianHtml will.
Make sure that the dummy path used in the run method (see `obsidianhtml_rust_module_example/module.py`) is a valid path for your system.