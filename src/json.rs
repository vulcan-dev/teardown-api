use std::{fs::File, io::Write};
use teardown_api::*;

pub fn gen(api: &API) {
    println!("Generating for Json...");

    let mut contents = String::from("{\n");
    for function in &api.function {
        let mut body = String::from(format!("{}(", function.name));
        if let Some(input) = &function.input {
            let mut i = 0;
            for param in input {
                body.push_str(&format!("\"${{{}:{}}}\", ", i+1, param.name));
                i += 1;
            }
            body.pop();
            body.pop();
            body.push_str(")");
        } else {
            body.push_str(&format!("{}()", function.name));
        }

        let mut output_len = 0;
        if let Some(output) = &function.output {
            output_len = output.len();
        }

        let mut input_len = 0;
        if let Some(input) = &function.input {
            input_len = input.len();
        }

        if input_len > 0 && output_len > 0 {
            contents.push_str(&format!("\t\"{}\": {{\n", function.name));
        } else {
            contents.push_str(&format!("\t\"{}\": {{", function.name));
        }

        if let Some(input) = &function.input { // args
            if output_len > 0 {
                contents.push_str(&format!("\t\t\"args\": [\n"));
            } else {
                contents.push_str(&format!("\n\t\t\"args\": [\n"));
            }

            let mut i = 0;
            for param in input {
                contents.push_str(&format!("\t\t\t{{\"name\": \"{}\", \"type\": \"{}\", \"description\": \"{}\", \"optional\": {}", param.name, param.type_, param.desc, param.optional));
                i = i + 1;

                if i < input_len {
                    contents.push_str("},\n");
                } else {
                    contents.push_str("}\n");
                }
            }

            if output_len > 0 {
                contents.push_str("\t\t],\n");
            }
            else {
                contents.push_str("\t\t]\n");
            }
        }

        if let Some(ret) = &function.output {
            if input_len > 0 {
                contents.push_str(&format!("\t\t\"return\": [\n"));
            } else {
                contents.push_str(&format!("\n\t\t\"return\": [\n"));
            }

            let mut i = 0;
            let ret_len = ret.len();
            for param in ret {
                contents.push_str(&format!("\t\t\t{{\"name\": \"{}\", \"type\": \"{}\", \"description\": \"{}\"", param.name, param.type_, param.desc));
                i = i + 1;

                if i < ret_len {
                    contents.push_str("},\n");
                } else {
                    contents.push_str("}\n");
                }
            }
            contents.push_str("\t\t]\n");
        }

        if output_len > 0 {
            contents.push_str("\t},\n");
        } else {
            contents.push_str("\t},\n");
        }
    }

    contents.pop();
    contents.pop();
    contents.push_str("\n}");

    let mut file = File::create("gen/Teardown.json").unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}