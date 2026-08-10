mod ast;
mod parser;
mod renderer;

use parser::Parser;
use renderer::HtmlRenderer;

use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "src/input.md".to_string());

    let markdown = fs::read_to_string(path)?;
    let ast = Parser::parse(&markdown);
    let html = HtmlRenderer::render(&ast);
    fs::write("output.html", html)?;

    println!("Parsed content saved in \"output.html\"");
    Ok(())
}
