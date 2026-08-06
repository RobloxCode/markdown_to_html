mod ast;
mod parser;
mod renderer;

use parser::Parser;
use renderer::HtmlRenderer;

use std::fs;

fn main() -> std::io::Result<()> {
    let markdown = fs::read_to_string("src/markdown.md")?;
    let ast = Parser::parse(&markdown);
    let html = HtmlRenderer::render(&ast);
    fs::write("parsed.html", html)?;

    println!("Parsed content saved in \"parsed.html\"");
    Ok(())
}
