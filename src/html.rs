use std::{fs::File, io::Write};
use teardown_api::*;

pub fn gen(api: &API) {
    println!("Generating for Html...");

    let mut contents = String::from("
<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>Teardown API</title>
</head>
<body>
<style>
body {
    font-family: 'Roboto', sans-serif;
    font-size: 14px;
    color: #fff;
    background-color: #282c34;
}

h1 {
    font-size: 1.5em;
    margin: 0;
    padding: 0;
}

h2 {
    font-size: 1.5em;
    margin: 0;
    padding: 0;
}

.func {
    margin: 0 auto;
    width: 40%;
    padding: 1em;
    background-color: #21252b;
    border-radius: 0.5em;
    box-shadow: 0 0.5em 1em rgba(0, 0, 0, 0.2);
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    margin-bottom: 1em;
}

.func:hover {
    background-color: #2b2f36;
}

table {
    width: 100%;
    border-collapse: collapse;
    border-spacing: 0;
    border: 1px solid #fff;
    border-radius: 0.5em;
    box-shadow: 0 0.5em 1em rgba(0, 0, 0, 0.2);
    margin: 0 auto;
    padding: 1em;
}

th {
    background-color: #21252b;
    color: #fff;
    text-align: left;
    padding: 0.5em;
    border-bottom: 1px solid #fff;
    font-weight: normal;
}

td {
    padding: 0.5em;
    border-bottom: 1px solid #fff;
}

a {
    color: #fff;
    text-decoration: none;
}
</style>
    ");

    for function in &api.function {
        contents.push_str(&format!("<a href=\"https://teardowngame.com/modding/api.html#{}\" target=\"_blank\"<div class=\"func\">", function.name));
        contents.push_str(&format!("<h1>{}</h1>", function.name));
        if let Some(input) = &function.input {
            contents.push_str("<h2>Input</h2>");
            contents.push_str("<table>");
            contents.push_str("<tr>");
            contents.push_str("<th>Name</th>");
            contents.push_str("<th>Type</th>");
            contents.push_str("<th>Optional</th>");
            contents.push_str("<th>Description</th>");
            contents.push_str("</tr>");
            for param in input {
                contents.push_str("<tr>");
                contents.push_str("<td>");
                contents.push_str(&format!("{}", param.name));
                contents.push_str("</td>");
                contents.push_str("<td>");
                contents.push_str(&format!("{}", param.type_));
                contents.push_str("</td>");
                contents.push_str("<td>");
                contents.push_str(&format!("{}", param.optional));
                contents.push_str("</td>");
                contents.push_str("<td>");
                contents.push_str(&format!("{}", param.desc));
                contents.push_str("</td>");
                contents.push_str("</tr>");
            }
            contents.push_str("</table>");
        }

        if let Some(output) = &function.output {
            contents.push_str("<h2>Output</h2>");
            contents.push_str("<table>");
            contents.push_str("<tr>");
            contents.push_str("<th>Name</th>");
            contents.push_str("<th>Type</th>");
            contents.push_str("<th>Description</th>");
            contents.push_str("</tr>");
            for param in output {
                contents.push_str("<tr>");
                contents.push_str("<td>");
                contents.push_str(&format!("{}", param.name));
                contents.push_str("</td>");
                contents.push_str("<td>");
                contents.push_str(&format!("{}", param.type_));
                contents.push_str("</td>");
                contents.push_str("<td>");
                contents.push_str(&format!("{}", param.desc));
                contents.push_str("</td>");
                contents.push_str("</tr>");
            }
            contents.push_str("</table>");
        }
        contents.push_str("</a></div>");
    }

    contents.push_str("</body>");
    contents.push_str("</html>");

    let mut file = File::create("gen/Teardown.html").unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}