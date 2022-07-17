use std::{fs::File, io::Write};
use teardown_api::*;

pub fn gen(api: &API) {
    println!("Generating for CSV...");

    let mut contents = String::from("Name, Arguments, Return\n");
    for function in &api.function {
        contents.push_str(&format!("{}, ", function.name));
        if let Some(input) = &function.input {
            contents.push_str("\"");
            for arg in input {
                if arg.optional {
                    contents.push_str(&format!("{} ({}, optional) - {}\n", arg.name, arg.type_, arg.desc));
                } else {
                    contents.push_str(&format!("{} ({}) - {}\n", arg.name, arg.type_, arg.desc));
                }
            }
            contents.push_str("\", ");
        }

        if let Some(output) = &function.output {
            contents.push_str("\"");
            for arg in output {
                contents.push_str(&format!("{} ({}) - {}\n", arg.name, arg.type_, arg.desc));
            }
            contents.push_str("\", ");
        }

        contents.push_str("\n");
    }

    let mut file = File::create("gen/api.csv").unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}