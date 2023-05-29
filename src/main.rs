//#[macro_use(out)]
extern crate obshtml;
extern crate yaml_rust;
extern crate json;
extern crate regex;

#[macro_use]
extern crate lazy_static;

extern crate linked_hash_map;

use std::path::{Path};

use json::JsonValue;

//use yaml_rust::{YamlEmitter}; //YamlLoader
use yaml_rust::Yaml;
use json::iterators::Members;
use linked_hash_map::LinkedHashMap;

use obshtml::{ObsidianModuleConfig, ObsidianModule};
use obshtml::module::options::{compile_default_options}; //get_configured_options
use obshtml::module::modfile::{compile_provides};
use obshtml::cli::execute;


use obshtml::stdlib::*;
use obshtml::lib::file;
use obshtml::lib::misc::{yaml_to_json};

mod metadata;
use metadata::{
    parse_frontmatter, 
    ensure_tags_item_in_frontmatter, 
    convert_tags_from_string_to_list, 
    get_inline_tags,
    insert_yaml_list_into_yaml_hash
};

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

    let provides = compile_provides(vec!("metadata.json"));

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
    // get files.json (we need the input folder later on)
    let paths_modfile = obsmod.modfile("paths.json");
    let contents = paths_modfile.read().unwrap();
    let paths = json::parse(&contents).unwrap();
    let input_folder_path = paths["input_folder"].as_str().unwrap();
    println!("{:?}", input_folder_path);

    // we will add all the metadata into this hashtable
    // using the rel_path as the key, and the metadata as the value
    let mut output_hash = json::JsonValue::new_object();

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

            // add inline_tags to the frontmatter tags list
            match &frontmatter["tags"] {
                Yaml::Array(inner) => {
                    let mut inner_cp = inner.clone();
                    for item in inline_tags {
                        inner_cp.push(Yaml::String(item.to_owned()));
                    }
                    inner_cp.sort();
                    inner_cp.dedup();
                    insert_yaml_list_into_yaml_hash(&mut frontmatter, "tags", Yaml::Array(inner_cp));
                },
                _ => (),
            }

            // get rel_path
            let abs_path = Path::new(file_path);
            let rel_path_res = abs_path.strip_prefix(input_folder_path);
            match rel_path_res {
                Err(inner) => {
                        obsmod.stderr("error", &format!("Failed to get relative path for rel({}, {}), first is not the parent of the second", input_folder_path, file_path));
                    },
                Ok(rel_path) => {
                    // write to output hash
                    let rel_path_str = rel_path.to_str().unwrap().to_owned();
                    output_hash[rel_path_str] = yaml_to_json(&frontmatter).unwrap();
                },
            }
        }
    }

    // println!("{:?}", output_hash);
    // let mut out_yaml_str = String::new();
    // let mut emitter = YamlEmitter::new(&mut out_yaml_str);
    // emitter.dump(&Yaml::Hash(output_hash)).unwrap(); 


    // write a output modfile
    let out_mod_file = obsmod.modfile("metadata.json");
    out_mod_file.write(&output_hash.pretty(2)).unwrap();

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