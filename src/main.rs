//#[macro_use(out)]
extern crate obshtml;
extern crate yaml_rust;

use yaml_rust::{YamlLoader, YamlEmitter};
use yaml_rust::Yaml;

use obshtml::{ObsidianModuleConfig, ObsidianModule};
use obshtml::module::options::{compile_default_options}; //get_configured_options
use obshtml::cli::execute;

fn main() {
    // define the default config options for this module that can be overwritten
    // by users in module_config: {}
    let default_options = compile_default_options("setting1: bla");

    // define module config
    let obs_cfg = ObsidianModuleConfig {
        module_name: "hello",
        module_class_name: "<crate::obshtml-example>",
        persistent: false,
        run_fn: run,
        accept_fn: accept,
        default_options: default_options,
    };

    execute::start(obs_cfg);
}

fn run(obsmod: ObsidianModule) {
    // get module options example
    let setting1 = obsmod.default_options["setting1"].as_str().unwrap();
    println!("{:?}", setting1);

    // return output
    // make sure to only output valid json to stdout when running as an actual module
    let output = r#"{"result": true}"#;
    println!("{}", output);

}

fn accept(obsmod: ObsidianModule) {
    //println!("inside accept_fn:\n\tmodule_data_folder: {}", module_data_folder);
    let output = r#"{"result": true}"#;
    println!("{}", output);
}