use std::{fs::File, io::Write};
use teardown_api::*;

pub fn gen(api: &API) {
    println!("Generating for Toml...");

    let mut contents = String::new();
    for function in &api.function {
        contents.push_str(&format!("[{}]\n", function.name));
        if let Some(input) = &function.input {
            contents.push_str("  args = [\n");
            for param in input {
                contents.push_str(&format!("    {{ name = \"{}\", type = \"{}\", optional = {}, desc = \"{}\" }},\n", param.name, param.type_, param.optional, param.desc));
            }
            contents.push_str("  ]\n");
        }

        if let Some(output) = &function.output {
            contents.push_str("  returns = [\n");
            for param in output {
                contents.push_str(&format!("    {{ name = \"{}\", type = \"{}\", desc = \"{}\" }},\n", param.name, param.type_, param.desc));
            }
            contents.push_str("  ]\n");
        }
    }

    let mut file = File::create("gen/Teardown.toml").unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}