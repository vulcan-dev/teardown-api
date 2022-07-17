use std::{fs::File, io::Write};
use teardown_api::*;

pub fn gen(api: &API) {
    println!("Generating for Sublime...");

    let mut contents = String::from("{\n");

    contents.push_str("\t\"scope\": \"source.lua\",\n");
    contents.push_str("\t\"completions\": [\n\t\t\"lua\",\n");


    for function in &api.function {
        let mut body = String::from(format!("{}(", function.name));

        if let Some(input) = &function.input {
            let mut i = 0;
            for param in input {
                body.push_str(&format!("${{{}:{}}}, ", i+1, param.name));
                i += 1;
            }
            body.pop();
            body.pop();
            body.push_str(")");
        } else {
            body.push_str(&format!("{}()", function.name));
        }

        contents.push_str(&format!("\t\t{{\"trigger\": \"{}\", \"contents\": \"{}\"}},\n", function.name, body));
    }

    contents.pop();
    contents.pop();
    contents.push_str("\t]\n}");

    let mut file = File::create("gen/Teardown.sublime-completions").unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}