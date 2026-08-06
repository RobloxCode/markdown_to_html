use crate::ast::{Ast, Node};

#[derive(Debug)]
pub struct Parser;

impl Parser {
    pub fn parse(markdown: &str) -> Ast {
        let mut document = Vec::new();
        let mut lines = markdown.lines().peekable();

        while let Some(line) = lines.next() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            if trimmed.starts_with('#') {
                let level = trimmed.chars().take_while(|&c| c == '#').count();
                let text = trimmed[level + 1..].to_string();

                document.push(Node::Heading { level, text });
            } else if trimmed.starts_with("- ") {
                let mut items = Vec::new();

                items.push(trimmed[2..].to_string());

                while let Some(&next) = lines.peek() {
                    let ntri = next.trim();

                    if ntri.starts_with("- ") {
                        items.push(ntri[2..].to_string());
                        lines.next();
                    } else {
                        break;
                    }
                }

                document.push(Node::List {
                    // ordered: false,
                    items,
                });

                continue;
            } else if trimmed.starts_with("```") {
                let language = trimmed[3..].to_string();
                let mut code_lines = String::new();

                while let Some(&next) = lines.peek() {
                    let ntri = next.trim();

                    if ntri.starts_with("```") {
                        lines.next();
                        break;
                    }

                    code_lines.push_str(ntri);
                    lines.next();
                }

                document.push(Node::Code {
                    language,
                    code_lines,
                });

                continue;
            } else if trimmed.starts_with('>') {
                document.push(Node::Quote(trimmed[2..].to_string()));
                continue;
            } else if trimmed.starts_with('|') {
                let mut rows = Vec::new();

                let headers = trimmed
                    .split('|')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
                    .collect::<Vec<_>>();

                lines.next();

                while let Some(&next) = lines.peek() {
                    let ntri = next.trim();

                    if !ntri.starts_with('|') {
                        break;
                    }

                    rows.push(
                        ntri.split('|')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>(),
                    );

                    lines.next();
                }

                document.push(Node::Table { headers, rows });
                continue;
            } else {
                document.push(Node::Paragraph(trimmed.to_string()));
                continue;
            }
        }

        Ast { document }
    }
}
