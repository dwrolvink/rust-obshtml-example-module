extern crate obshtml;
extern crate yaml_rust;
extern crate json;
extern crate regex;
extern crate linked_hash_map;

#[macro_use]
extern crate lazy_static;

// data structures
use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;
use json::JsonValue;
use json::iterators::Members;

// obshtml elements
use obshtml::{ObsidianModuleConfig, ObsidianModule};
use obshtml::module::options::{compile_default_options}; //get_configured_options
use obshtml::module::modfile::{compile_provides};
use obshtml::cli::execute;

// helper functions/objects
use obshtml::stdlib::*;
use obshtml::lib::file;
use obshtml::lib::misc::{yaml_to_json};
use std::path::{Path};

// internal modules
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
    let default_options = compile_default_options("test: set this value to test options being set");

    // list files that this module will create/alter
    let provides = compile_provides(vec!("index/metadata.json"));

    // TODO: add requires!

    // define module config
    let obs_cfg = ObsidianModuleConfig {
        module_name: "hello",
        module_class_name: "<crate::obshtml-example>",
        run_fn: run,
        accept_fn: accept,
        default_options: default_options,
        provides: provides,
    };

    execute::start(obs_cfg);
}

fn run(obsmod: ObsidianModule) {
    /*
        This function is the "meat" of this module. 
        It will get the modfile index/files.json, and for each markdown file parse the frontmatter yaml 
        and get the inline tags.
        It then merges the tags listed in the frontmatter, and the found inline tags, removing duplicates,
        making sure that the metadata["tags"] key is set with an empty list if no tags are found at all.

        The result is written to modfile index/metadata.json with structure:
        {
            "$REL_PATH": {$METADATA},
            "$REL_PATH": {$METADATA},
            ...
        }
    */

    // Get paths.json and then get paths["input_folder"]
    // We need this value to get the rel_path of the files (index/files.json has absolute paths)
    let paths_modfile = obsmod.modfile("paths.json");
    let contents = paths_modfile.read().unwrap();
    let paths = json::parse(&contents).unwrap();
    let input_folder_path = paths["input_folder"].as_str().unwrap();

    // We will add all the metadata into this hashtable as we collect it.
    // Using the rel_path as the key, and the metadata as the value
    let mut output_hash = json::JsonValue::new_object();

    // Read index/files.json and read the file paths in the list
    let mod_file = obsmod.modfile("index/files.json");
    obsmod.stderr("debug", &format!("abs path of modfile: {}", &mod_file.get_abs_file_path()));

    let contents = mod_file.read().unwrap();
    let file_list = json::parse(&contents).unwrap();
    for item in file_list.members() {
        let file_path = item.as_str().unwrap();

        // get the frontmatter from markdown files
        if file_path.ends_with(".md") {
            obsmod.stderr("debug", &format!("getting metadata for: {}", file_path));

            let mut frontmatter = parse_frontmatter(&obsmod, file_path);
            ensure_tags_item_in_frontmatter(&mut frontmatter);
            convert_tags_from_string_to_list(&mut frontmatter);

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

            // this is our final frontmatter
            obsmod.stderr("debug", &format!("    {:?}", frontmatter));

            // get rel_path and write to output_hash
            let abs_path = Path::new(file_path);
            let rel_path_res = abs_path.strip_prefix(input_folder_path);
            match rel_path_res {
                Err(inner) => {
                    // failed to get rel_path
                        obsmod.stderr("error", &format!("Failed to get relative path for rel({}, {}), first is not the parent of the second", input_folder_path, file_path));
                    },
                Ok(rel_path) => {
                    // we got the rel_path, now we can write the metadata to the output_hash
                    let rel_path_str = rel_path.to_str().unwrap().to_owned();
                    output_hash[rel_path_str] = yaml_to_json(&frontmatter).unwrap();
                },
            }
        }
    }

    // write the output modfile
    let out_mod_file = obsmod.modfile("index/metadata.json");
    out_mod_file.write(&output_hash.pretty(2)).unwrap();

    // return output
    // make sure to only output valid json to stdout when running as an actual module
    let output = r#"{"result": true}"#;
    println!("{}", output);
}


fn accept(_obsmod: ObsidianModule) {
    /*
        This function is called by ObsidianHtml to test if we need to run the module proper
        In our current case, we always want to run this module when configured
    */
    let output = r#"{"result": true}"#;
    println!("{}", output);
}