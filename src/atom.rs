use std::{fs::File, io::Write};
use teardown_api::*;

pub fn gen(api: &API) {
    println!("Generating for Atom...");

    let mut contents = String::from("'.source.lua':\n");
    for function in &api.function {
        let mut body = String::from(format!("{}(", function.name));

        if let Some(input) = &function.input {
            let mut i = 0;
            for param in input {
                if param.type_ == "string" {
                    body.push_str(&format!("\"${{{}:{}}}\", ", i+1, param.name));
                } else {
                    body.push_str(&format!("${{{}:{}}}, ", i+1, param.name));
                }
                i += 1;
            }
            body.pop();
            body.pop();
            body.push_str(")");
        } else {
            body.push_str(")");
        }

        contents.push_str(&format!("  '{}':\n", function.name));
        contents.push_str(&format!("    'prefix': '{}',\n", function.name));
        contents.push_str(&format!("    'body': '{}'\n", body));
    }

    let mut file = File::create("gen/Teardown.completions.cson").unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}