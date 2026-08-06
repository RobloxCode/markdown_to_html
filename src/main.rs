mod ast;
mod parser;
mod renderer;

use parser::Parser;
use renderer::HtmlRenderer;

use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let path: String;
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        path = args[1].clone();
    } else {
        path = "src/markdown.md".to_string();
    }

    let markdown = fs::read_to_string(path)?;
    let ast = Parser::parse(&markdown);
    let html = HtmlRenderer::render(&ast);
    fs::write("parsed.html", html)?;

    println!("Parsed content saved in \"parsed.html\"");
    Ok(())
}
