use serde::{self, Deserialize};

#[derive(Debug, Deserialize)]
pub struct Input {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub optional: bool,
    pub desc: String,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub desc: String,
}

#[derive(Debug, Deserialize)]
pub struct Function {
    pub name: String,
    pub input: Option<Vec<Input>>,
    pub output: Option<Vec<Output>>,
}

#[derive(Debug, Deserialize)]
pub struct API {
    pub function: Vec<Function>,
}