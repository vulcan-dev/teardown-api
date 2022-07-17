use std::{fs::File, io::Write};
use teardown_api::*;

pub fn gen(api: &API) {
    let mut contents = String::from("# Teardown API\n\n");

    for function in &api.function {
        contents.push_str(&format!("## {}\n\n", function.name));

        if let Some(input) = &function.input {
            contents.push_str(&format!("### Input\n\n"));
            contents.push_str(&format!("| Name | Type | Optional | Description |\n"));
            contents.push_str(&format!("| ---- | ---- | -------- | ----------- |\n"));
            for param in input {
                contents.push_str(&format!("| {} | {} | {} | {} |\n", param.name, param.type_, param.optional, param.desc));
            }
        } else {
            contents.push_str(&format!("### Input\n\n"));
            contents.push_str(&format!("No input\n\n"));
        }

        if let Some(output) = &function.output {
            contents.push_str(&format!("### Output\n\n"));
            contents.push_str(&format!("| Name | Type | Description |\n"));
            contents.push_str(&format!("| ---- | ---- | ----------- |\n"));
            for param in output {
                contents.push_str(&format!("| {} | {} | {} |\n", param.name, param.type_, param.desc));
            }
        }
    }

    let mut file = File::create("gen/Teardown.md").unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}