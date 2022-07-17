use std::{fs::File, io::Write};
use teardown_api::*;

pub fn gen(api: &API) {
    &println!("Generating for Markdown...");

    let mut contents = String::from("# Teardown API\n\n");

    for function in &api.function {
        contents.push_str(&format!("## {}\n\n", function.name));

        if let Some(input) = &function.input {
            contents.push_str(&format!("### Input\n\n"));
            contents.push_str(&format!("| Name | Type |\n"));
            contents.push_str(&format!("| ---- | ---- |\n"));
            for param in input {
                contents.push_str(&format!("| {} | {} |\n", param.name, param.type_));
            }
        }

        if let Some(output) = &function.output {
            contents.push_str(&format!("### Output\n\n"));
            contents.push_str(&format!("| Name | Type |\n"));
            contents.push_str(&format!("| ---- | ---- |\n"));
            for param in output {
                contents.push_str(&format!("| {} | {} |\n", param.name, param.type_));
            }
        }
    }

    let mut file = File::create("gen/api.md").unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}