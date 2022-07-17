use std::{fs::File, io::Write, env};

use reqwest;
use quick_xml;
use serde::{self, Deserialize};

#[derive(Debug, Deserialize)]
struct Input {
    name: String,
    #[serde(rename = "type")]
    type_: String,
    optional: bool,
    desc: String,
}

#[derive(Debug, Deserialize)]
struct Output {
    name: String,
    #[serde(rename = "type")]
    type_: String,
    desc: String,
}

#[derive(Debug, Deserialize)]
struct Function {
    name: String,
    input: Option<Vec<Input>>,
    output: Option<Vec<Output>>,
}

#[derive(Debug, Deserialize)]
struct API {
    function: Vec<Function>,
}

async fn get_xml() -> String {
    let response = reqwest::get("https://teardowngame.com/modding/api.xml").await.unwrap();
    let body = response.text().await.unwrap();
    body
}

fn get_body(function: &Function) -> String {
    let mut body = String::new();

    // check if input exists
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

fn gen_vscode(api: &API) {
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

    let mut file = File::create("Teardown.code-snippets").unwrap();
    file.write_all(out_str.as_bytes()).unwrap();
}

fn gen_subl(api: API) {
    let mut contents = String::from("{\n");

    contents.push_str("\t\"scope\": \"source.lua\",\n");
    contents.push_str("\t\"completions\": [\n\t\t\"lua\",\n");


    for function in api.function {
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

    let mut file = File::create("Teardown.sublime-completions").unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

fn gen_atom(api: API) {
    let mut contents = String::from("'.source.lua':\n");
    for function in api.function {
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

    let mut file = File::create("Teardown.completions.cson").unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: [-subl | -vscode | -atom]");
        return;
    }

    let content = get_xml().await;
    let api: API = quick_xml::de::from_str(&content).unwrap_or_else(|e| panic!("{}", e));

    if args[1] == "-vscode" {
        println!("Generating vscode snippets");
        gen_vscode(&api);
    } else if args[1] == "-subl" {
        println!("Generating subl snippets");
        gen_subl(api);
    } else if args[1] == "-atom" {
        println!("Generating atom snippets");
        gen_atom(api);
    } else {
        println!("Usage: [-subl | -vscode | -atom]");
    }
}