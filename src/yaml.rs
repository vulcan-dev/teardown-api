use std::{fs::File, io::Write};
use teardown_api::*;

pub fn gen(api: &API) {
    println!("Generating for Yaml...");

    let mut contents = String::new();
    for function in &api.function {
        contents.push_str(&format!("{}:\n", function.name));
        if let Some(input) = &function.input {
            contents.push_str("  args:\n");
            for param in input {
                contents.push_str(&format!("    - name: {}\n", param.name));
                contents.push_str(&format!("      type: \"{}\"\n", param.type_));
                contents.push_str(&format!("      optional: {}\n", param.optional));
                contents.push_str(&format!("      description: \"{}\"\n", param.desc));
            }
        }

        if let Some(output) = &function.output {
            contents.push_str("  returns:\n");
            for param in output {
                contents.push_str(&format!("    - name: {}\n", param.name));
                contents.push_str(&format!("      type: \"{}\"\n", param.type_));
                contents.push_str(&format!("      description: \"{}\"\n", param.desc));
            }
        }
    }

    let mut file = File::create("gen/Teardown.yaml").unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}