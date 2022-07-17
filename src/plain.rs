use std::{fs::File, io::Write};
use teardown_api::*;

pub fn gen(api: &API) {
    let mut contents = String::new();

    for function in &api.function {
        contents.push_str(&format!("---- {} ----\n", function.name));

        if let Some(input) = &function.input {
            contents.push_str("  [Input]\n");
            for param in input {
                if param.optional {
                    contents.push_str(&format!("    {} ({}, optional) - {}\n", param.name, param.type_, param.desc));
                } else {
                    contents.push_str(&format!("    {} ({}) - {}\n", param.name, param.type_, param.desc));
                }
            }

            contents.push_str("\n");
        }

        if let Some(output) = &function.output {
            contents.push_str("  [Output]\n");
            for param in output {
                contents.push_str(&format!("    {} ({}) - {}\n", param.name, param.type_, param.desc));
            }

            contents.push_str("\n");
        }
    }

    let mut file = File::create("gen/Teardown.txt").unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}