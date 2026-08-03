#[derive(Debug)]
enum Node {
    Heading { level: usize, text: String },
    Paragraph(String),
    List { ordered: bool, items: Vec<String> },
    Code(String),
    Quote(String),
    HorizontalRule,
}

#[derive(Debug)]
struct Ast {
    document: Vec<Node>,
}

fn parse(input: &str) -> Ast {
    let document = Vec::new();

    Ast { document }
}

fn main() {}
