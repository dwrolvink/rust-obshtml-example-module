//#[macro_use(out)]
extern crate obshtml;

use obshtml::ObsidianModuleConfig;
use obshtml::cli::execute;

fn main() {
    let obs_cfg = ObsidianModuleConfig {
        module_name: "hello",
        module_class_name: "<crate::obshtml-example>",
        persistent: false,
        run_fn: run,
        accept_fn: accept,
    };

    execute::start(obs_cfg);
}

fn run(module_data_folder: String) {
    println!("inside run_fn:\n\tmodule_data_folder: {}", module_data_folder);
}

fn accept(module_data_folder: String) {
    println!("inside accept_fn:\n\tmodule_data_folder: {}", module_data_folder);
}