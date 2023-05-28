use std::fs::File;
use std::io::{ self, BufRead, BufReader };

use lazy_static::lazy_static;
use regex::Regex;

use yaml_rust::YamlLoader;
use yaml_rust::Yaml;
use yaml_rust::yaml::Hash;

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

pub fn get_inline_tags(contents: &str) -> Vec<String> {
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

// def get_inline_tags(self, page):
//     return [x[1:].replace(".", "") for x in re.findall(r"(?<!\S)#[\w/\-]*[a-zA-Z\-_/][\w/\-]*", page)]

// def sanatize_frontmatter(self, metadata):
//     # imitate obsidian shenannigans
//     if "tags" in metadata.keys():
//         tags = metadata["tags"]
//         if isinstance(tags, str):
//             if " " in tags.strip() or "," in tags:
//                 metadata["tags"] = [x.rstrip(",") for x in tags.replace(",", " ").split(" ") if x != ""]
//             elif tags.strip() == "":
//                 metadata["tags"] = []
//             else:
//                 metadata["tags"] = [tags,]
//         elif tags is None:
//             metadata["tags"] = []
//     else:
//         metadata["tags"] = []
//     return metadata

// def get_frontmatter(self, file_path):
//     with open(file_path, encoding="utf-8") as f:
//         metadata, page = frontmatter.parse(f.read())
//     return self.sanatize_frontmatter(metadata), page


