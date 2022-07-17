mod vscode;
mod sublime;
mod atom;
mod json;
mod md;
mod csv;
mod toml;
mod yaml;
mod html;
mod plain;

use std::{env, collections::HashMap};

use reqwest;
use quick_xml;
use teardown_api::*;

async fn get_xml() -> String {
    let response = reqwest::get("https://teardowngame.com/modding/api.xml").await.unwrap();
    let body = response.text().await.unwrap();
    body
}

#[tokio::main]
async fn main() {
    let mut functions: HashMap<&str, fn(&API)> = HashMap::new();
    functions.insert("sublime", sublime::gen);
    functions.insert("vscode", vscode::gen);
    functions.insert("atom", atom::gen);
    functions.insert("json", json::gen);
    functions.insert("md", md::gen);
    functions.insert("csv", csv::gen);
    functions.insert("toml", toml::gen);
    functions.insert("yaml", yaml::gen);
    functions.insert("html", html::gen);
    functions.insert("plain", plain::gen);

    let avail_generators = "vscode | sublime | atom | json | toml | yaml | md | csv | html | plain";

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Invalid arguments. Usage: .\\teardown-api.exe [{}]", avail_generators);
        return;
    }

    let content = get_xml().await;
    let api: API = quick_xml::de::from_str(&content).unwrap_or_else(|e| panic!("{}", e));

    let gen_dir = std::path::Path::new("gen");
    if !gen_dir.exists() {
        std::fs::create_dir_all(gen_dir).unwrap();
    }

    let start_time = std::time::Instant::now();

    for arg in &args[1..] {
        if let Some(func) = functions.get(arg.as_str()) {
            func(&api);
        } else {
            println!("Unknown generator: {}. Available generators: [{}]", arg, avail_generators);
            return
        }
    }

    println!("Generated in {}ms", start_time.elapsed().as_millis());
}