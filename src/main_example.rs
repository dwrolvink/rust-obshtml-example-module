/*
    This is a simple example.
    Rename main.rs to main_bak.rs (or remove), and rename this file to main.rs to run the example.
*/

//#[macro_use(out)]
extern crate obshtml;
extern crate yaml_rust;
extern crate json;

// use yaml_rust::{YamlLoader, YamlEmitter};
// use yaml_rust::Yaml;
use json::object;

use obshtml::{ObsidianModuleConfig, ObsidianModule};
use obshtml::module::options::{compile_default_options}; //get_configured_options
use obshtml::module::modfile::{compile_provides};
use obshtml::cli::execute;

use obshtml::stdlib::*;

fn main() {
    // define the default config options for this module that can be overwritten
    // by users in module_config: {}
    let default_options = compile_default_options("
a:
    a1: old (should be overwritten)
    a2: old (should not be overwritten)
b: old (should be overwritten)
c: old (only in default)
        ");

    let provides = compile_provides(vec!("test.json"));

    // define module config
    let obs_cfg = ObsidianModuleConfig {
        module_name: "hello",
        module_class_name: "<crate::obshtml-example>",
        persistent: false,
        run_fn: run,
        accept_fn: accept,
        default_options: default_options,
        provides: provides,
    };

    execute::start(obs_cfg);
}

fn run(obsmod: ObsidianModule) {

    // example of how to get the module options
    // it also shows the returned values and how to unpack them
    // *note also the eprintln, we should only print valid json to stdout!*
    let val = &obsmod.options["a"];
    eprintln!("Debug: {}< {:?} >", get_type_of(val), val, );

    let val = &obsmod.options["b"];
    eprintln!("Debug: {}< {:?} >", get_type_of(val), val, );

    let val = &obsmod.options["b"].as_str().unwrap();
    eprintln!("Debug: {}< {:?} >", get_type_of(val), val, );

    // write a random modfile
    let mod_file1 = obsmod.modfile("test.json");

    let data = object!{
        foo: false,
        bar: null,
        answer: 42,
        list: [null, "world", true]
    };   

    mod_file1.write(&data.pretty(2)).unwrap();

    // get path of modfile
    eprintln!("Debug: abs path of modfile: {}", &mod_file1.get_abs_file_path());

    // read a random modfile
    let mod_file2 = obsmod.modfile("test.json");
    eprintln!("Debug: test.json contents:\n{}", mod_file2.read().unwrap());

    
    // return output
    // make sure to only output valid json to stdout when running as an actual module
    let output = r#"{"result": true}"#;
    println!("{}", output);
}


fn accept(_obsmod: ObsidianModule) {
    //println!("inside accept_fn:\n\tmodule_data_folder: {}", module_data_folder);
    let output = r#"{"result": true}"#;
    println!("{}", output);
}