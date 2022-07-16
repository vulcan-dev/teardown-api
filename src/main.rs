use std::{fs::File, io::Write};

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

/* Example function
<function name="GetBoolParam">
    <input name="name" type="string" optional="false" desc="Parameter name"/>
    <input name="default" type="boolean" optional="false" desc="Default parameter value"/>
    <output name="value" type="boolean" desc="Parameter value"/>
</function>
*/

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
        let len = input.len();
        for param in input {
            let mut name = param.name.clone();
            let type_ = param.type_.clone();
            let optional = param.optional;

            if optional {
                name = format!("opt_{}", name);
            }

            match type_.as_str() {
                "string" => {
                    if i == 0 {
                        body.push_str(format!("\\\"${{1:{}}}\\\"", name).as_str());
                    } else if i == len-1 {
                        body.push_str(format!("\\\"${{0:{}}}\\\"", name).as_str());
                    } else {
                        body.push_str(format!("\\\"${{{}:{}}}\\\"", i+1, name).as_str());
                    }
                },
                _ => {
                    if i == 0 {
                        body.push_str(format!("${{1:{}}}", name).as_str())
                    } if i == len-1 {
                        body.push_str(format!("${{0:{}}}", name).as_str())
                    } else {
                        body.push_str(format!("${{{}:{}}}", i+1, name).as_str())
                    }
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
                desc.push_str(&format!("{} ({}, optional)\\n", param.type_, param.name));
            } else {
                desc.push_str(&format!("{} ({})\\n", param.type_, param.name));
            }
        }
    } else {
        desc.push_str("None\\n");
    }

    desc.push_str("\\nReturns\\n");
    if let Some(output) = &function.output {
        for param in output {
            desc.push_str(&format!("{} {}\\n", param.type_, param.name));
        }
    } else {
        desc.push_str("None");
    }

    desc
}

#[tokio::main]
async fn main() {
    let content = get_xml().await;
    let api: API = quick_xml::de::from_str(&content).unwrap_or_else(|e| panic!("{}", e));

    let mut contents = Vec::new();
    contents.push(String::from("{\n"));

    for function in api.function {
        let mut body: Vec<String> = Vec::new();
        body.push(get_body(&function));
        // body.push(format!("\"{}(\\\"$1\\\")\"", function.name));

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
    
    let mut file = File::create("api.code-snippets").unwrap();
    file.write_all(out_str.as_bytes()).unwrap();
}