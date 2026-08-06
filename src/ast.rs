#[derive(Debug)]
pub enum Node {
    Heading {
        level: usize,
        text: String,
    },

    Paragraph(String),

    List {
        // TODO: have to be able to check when the list is ordered (1., 2., 3., ...)
        // ordered: bool,
        items: Vec<String>,
    },

    Code {
        language: String,
        code_lines: String,
    },

    Quote(String),

    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
}

#[derive(Debug)]
pub struct Ast {
    pub document: Vec<Node>,
}
