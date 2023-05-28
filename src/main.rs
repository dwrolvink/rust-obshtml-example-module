//#[macro_use(out)]
extern crate obshtml;
extern crate yaml_rust;
extern crate json;
extern crate regex;

#[macro_use]
extern crate lazy_static;

// use yaml_rust::{YamlLoader, YamlEmitter};
use yaml_rust::Yaml;
//use json::object;
use json::iterators::Members;

use obshtml::{ObsidianModuleConfig, ObsidianModule};
use obshtml::module::options::{compile_default_options}; //get_configured_options
use obshtml::module::modfile::{compile_provides};
use obshtml::cli::execute;

use obshtml::stdlib::*;
use obshtml::lib::file;

mod metadata;
use metadata::{parse_frontmatter, ensure_tags_item_in_frontmatter, convert_tags_from_string_to_list, get_inline_tags};

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

    // read index/files.json and read the file paths in the list
    let mod_file = obsmod.modfile("index/files.json");
    obsmod.stderr("debug", &format!("abs path of modfile: {}", &mod_file.get_abs_file_path()));

    let contents = mod_file.read().unwrap();
    let file_list = json::parse(&contents).unwrap();
    for item in file_list.members() {
        let file_path = item.as_str().unwrap();

        // get the frontmatter from markdown files
        if file_path.ends_with(".md") {
            obsmod.stderr("debug", &format!("getting frontmatter for: {}", file_path));

            let mut frontmatter = parse_frontmatter(&obsmod, file_path);
            ensure_tags_item_in_frontmatter(&mut frontmatter);
            convert_tags_from_string_to_list(&mut frontmatter);

            obsmod.stderr("debug", &format!("    {:?}", frontmatter));

            let contents = file::read(file_path).unwrap();
            let inline_tags = get_inline_tags(&contents);
            // if inline_tags.len() > 0 {
            //     obsmod.stderr("debug", &format!("    inline tags: {:?}", inline_tags));
            // }
        }
    }



    // // write a random modfile
    // let mod_file1 = obsmod.modfile("test.json");

    // let data = object!{
    //     foo: false,
    //     bar: null,
    //     answer: 42,
    //     list: [null, "world", true]
    // };   

    // mod_file1.write(&data.pretty(2)).unwrap();

    
    // obsmod.stderr("debug", &format!("{}< {:?} >", get_type_of(&it), it));
    //println!("{:?}", file_paths);
    





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