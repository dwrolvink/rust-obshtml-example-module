use std::fs::File;
use std::io::{ self, BufRead, BufReader };

use lazy_static::lazy_static;
use regex::Regex;

use yaml_rust::YamlLoader;
use yaml_rust::Yaml;
use yaml_rust::yaml::Hash;

use obshtml::stdlib::*;
use obshtml::{ObsidianModule};


fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap(); 
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines(); 
}

pub fn parse_frontmatter(obsmod: &ObsidianModule, file_path: &str) -> Yaml {
    /*
        enter yaml:
          if first non-empty line = "---" (no whitespace allowed) -> enter yaml
          else -> return none
        exit yaml:
          if line = "---" (no whitespace allowed) -> exit yaml        
        in yaml:
          write line to text with newline
        on exit:
          parse as yaml
          on parse as yaml:
            if unwrap fails, print error with filepath and return none
            else return yaml
    */

    let mut first_line = true;
    let mut text = String::from("");

    let lines = read_lines(file_path.to_string());
    for line_opt in lines {
        // handle line opt
        let line;
        match line_opt {
            Err(_) => break,
            Ok(inner) => line = inner,
        }

        // enter yaml block or conclude no such block exists
        if first_line {
            // skip leading empty lines
            if line.len() == 0 {
                continue;
            }

            first_line = false;
            
            if line == "---" {
                continue;
            }
            
            // no yaml block in page
            return Yaml::Null;
        }
        // exit yaml block
        if line == "---" {
            break;
        }
        
        // only other option here is that we are in the yaml block
        // add current line to text
        text.push('\n');
        text.push_str(&line);

        // println!("- {}", line.unwrap());
    }

    // if text is empty, return none
    if text.len() == 0 {
        return Yaml::Null;
    }

    // try parse yaml
    let docs_opt = YamlLoader::load_from_str(&text);
    match docs_opt {
        Err(_) => {
            obsmod.stderr("error", &format!("failed to parse frontmatter in file {}", file_path));
            return Yaml::Null;
        }
        Ok(inner) => {
            let doc = &inner[0];
            return doc.clone();
        }
    }
}

pub fn ensure_tags_item_in_frontmatter(mut metadata: &mut Yaml) {
    /* 
        If no frontmatter is present, just set it to `tags: []`
        This function ensures that the ITEM "tags" is present.
        In obsidian, you can set tags: "somestring values", this string is converted to a list 
        in convert_tags_from_string_to_list(), not here.
    */
    if *metadata == Yaml::Null {
        let empty_tags_doc = YamlLoader::load_from_str("tags: []").unwrap();
        *metadata = empty_tags_doc[0].clone();
        return;
    }

    // if frontmatter is present, check that `tags: [x]` is set, if not, add it
    if variant_eq(&metadata["tags"], &Yaml::BadValue) {
        let empty_list = Yaml::Array([].to_vec());
        insert_yaml_list_into_yaml_hash(&mut metadata, "tags", empty_list);
    }
}

pub fn insert_yaml_list_into_yaml_hash(mut yaml_hash: &mut Yaml, key: &str, list: Yaml) {
    // unpack the inner hash value from the yaml
    match &mut yaml_hash {
        Yaml::Hash(inner) => {
            // add the empty empty list at the provided key 
            inner.insert(Yaml::String(key.to_string()), list);
        },
        _ => {
            panic!("Expected Yaml::Hash, got something else");
        },
    }
}

pub fn convert_tags_from_string_to_list(mut metadata: &mut Yaml) {
    /* 
        In obsidian, you can set tags: "somestring values", this string is converted to a list here.
        First run ensure_tags_item_in_frontmatter to ensure that Yaml is not Yaml::Null!
    */
    if variant_eq(&metadata["tags"], &Yaml::String("".to_string())) {

        // unpack inner value
        match &metadata["tags"] {
            Yaml::String(inner) => {
                if inner.trim().len() == 0 {
                    // tags: "" || tags: "  " -> tags: []
                    let empty_list = Yaml::Array([].to_vec());
                    insert_yaml_list_into_yaml_hash(&mut metadata, "tags", empty_list);
                }
                else {
                    // replace , with " " and then split on spaces, removing empty items
                    // replace string value with new list
                    let mut tags_vec = [].to_vec();

                    let tags_str = inner.replace(",", " ");
                    for part in tags_str.split(" ") {
                        if part.trim().len() > 0 {
                            tags_vec.push(Yaml::String(part.to_string()));
                        }
                    }

                    insert_yaml_list_into_yaml_hash(&mut metadata, "tags", Yaml::Array(tags_vec));                 
                }
            },
            _ => (),
        }
    }
}

pub fn get_inline_tags(contents: &str) -> Vec<String> {
    /*
        TODO: FIRST strip codeblocks/lines!!
    */
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\s|^)#[\w/\-]*[a-zA-Z\-_/][\w/\-]*").unwrap();
    }
    
    let mut tags = Vec::new();
    for cap in RE.captures_iter(contents) {
        let mut tag = cap[0].trim().to_owned();
        tag = tag.replace("#", "");
        tags.push(tag);
    }

    return tags;
}



