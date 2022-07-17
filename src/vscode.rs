use std::{fs::File, io::Write};
use teardown_api::*;

fn get_body(function: &Function) -> String {
    let mut body = String::new();

    if let Some(input) = &function.input {
        body.push_str(&format!("\"{}(", function.name));

        let mut i = 0;
        for param in input {
            let mut name = param.name.clone();
            let type_ = param.type_.clone();
            let optional = param.optional;

            if optional {
                name = format!("opt_{}", name);
            }

            match type_.as_str() {
                "string" => {
                    body.push_str(format!("\\\"${{{}:{}}}\\\"", i+1, name).as_str());
                },
                _ => {
                    body.push_str(format!("${{{}:{}}}", i+1, name).as_str())
                }
            }

            i += 1;

            body.push_str(", ");
        }
        body.pop();
        body.pop();

        body.push_str(")\"");
    } else {
        body.push_str(&format!("\"{}()\"", function.name));
    }

    body
}

fn gen_desc(function: &Function) -> String {
    let mut desc = String::from("Arguments\\n");

    if let Some(input) = &function.input {
        for param in input {
            if param.optional {
                desc.push_str(&format!("{} ({}, optional) - {}\\n", param.type_, param.name, param.desc));
            } else {
                desc.push_str(&format!("{} ({}) - {}\\n", param.type_, param.name, param.desc));
            }
        }
    } else {
        desc.push_str("None\\n");
    }

    desc.push_str("\\nReturns\\n");
    if let Some(output) = &function.output {
        for param in output {
            desc.push_str(&format!("{} {} - {}\\n", param.type_, param.name, param.desc));
        }
    } else {
        desc.push_str("None");
    }

    desc
}

pub fn gen(api: &API) {
    println!("Generating for VSCode...");

    let mut contents = Vec::new();
    contents.push(String::from("{\n"));

    for function in &api.function {
        let mut body: Vec<String> = Vec::new();
        body.push(get_body(&function));

        contents.push(format!("\t\"{}\": {{\n", function.name));
        contents.push(String::from("\t\t\"scope\": \"lua\",\n"));
        contents.push(format!("\t\t\"prefix\": \"{}\",\n", function.name));

        contents.push(format!("\t\t\"body\": [\n"));
        contents.push(String::from("\t\t\t"));
        contents.push(body.join(",\n\t\t\t"));
        contents.push(String::from("\n\t\t],\n"));

        contents.push(format!("\t\t\"description\": \"{}\"\n", gen_desc(&function)));

        contents.push(String::from("\t},\n"));
    }
    contents.push(String::from("\n}"));

    let mut out_str = String::new();
    for line in contents {
        out_str.push_str(&line);
    }

    let mut file = File::create("gen/Teardown.code-snippets").unwrap();
    file.write_all(out_str.as_bytes()).unwrap();
}