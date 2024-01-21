use std::collections::HashMap;
use std::io::{self, stdout, Write};
use std::process::exit;

use anyhow::Result;
use serde;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum PossibleOption<'a> {
    T(&'a str),
    B(bool),
    C(&'a Vec<isize>),
}

#[derive(Debug, Deserialize)]
struct OllamaRes {
    // model: String,
    // created_at: String,
    response: String,
    context: Vec<isize>,
    // prompt_eval_count: isize,
    // prompt_eval_duration: isize,
    // eval_count: isize,
    // eval_duration: isize,
}

fn read_prompt() -> Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?;

    match buffer.trim() {
        "q" => exit(0),
        _ => Ok(buffer),
    }
}

fn run_query(prompt: &str, ctx: &Vec<isize>) -> Result<OllamaRes> {
    let mut map: HashMap<&str, PossibleOption> = HashMap::new();
    map.insert("model", PossibleOption::T("mistral"));
    map.insert("prompt", PossibleOption::T(prompt));
    map.insert("stream", PossibleOption::B(false));
    map.insert("context", PossibleOption::C(ctx));
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post("http://localhost:11434/api/generate")
        .json(&map)
        .send()?;

    let output: OllamaRes = resp.json()?;

    Ok(output)
}
fn main() -> Result<()> {
    let mut prompts: Vec<isize> = Vec::new();

    println!("Give me the info I need, and make it snappy.");
    println!("I don't have time for chit-chat or niceties.");
    println!("");
    println!("Type `q` to quit.");
    loop {
        print!("\nPrompt => ");
        stdout().flush()?;

        let prompt = read_prompt()?;
        println!("\nRunning. Give me some time.\n");
        let res = run_query(&prompt, &prompts)?;
        println!("{}", &res.response);
        prompts = res.context.clone();
    }
}
