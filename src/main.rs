#[derive(Debug)]
enum Node {
    Heading {
        level: usize,
        text: String,
    },

    Paragraph(String),

    List {
        // TODO: have to be able to check when the list is ordered (1., 2., 3., ...)
        ordered: bool,
        items: Vec<String>,
    },

    Code {
        language: String,
        code_lines: String,
    },

    Quote(String),

    Table {
        headers: Vec<Vec<String>>,
        rows: Vec<Vec<String>>,
    },
}

#[derive(Debug)]
struct Ast {
    document: Vec<Node>,
}

fn parse(input: &str) -> Ast {
    let mut document = Vec::new();
    let mut lines = input.lines().peekable();

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
                ordered: false,
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
            let mut headers = Vec::new();
            let mut rows = Vec::new();

            headers.push(
                trimmed
                    .split('|')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
                    .collect::<Vec<_>>(),
            );

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

fn render(ast: &Ast) -> String {
    let mut html = String::new();

    for item in ast.document.iter() {
        match item {
            Node::Heading { level, text } => {
                html.push_str(&format!("<h{level}>{text}</h<{level}>"))
            }
            Node::Paragraph(t) => html.push_str(&format!("<p>{t}</p>")),
            Node::List { ordered: _, items } => {
                html.push_str(&format!("<ul>"));

                for s in items {
                    html.push_str(&format!("<li>{s}</li>"));
                }

                html.push_str(&format!("</ul>"));
            }
            Node::Code {
                language,
                code_lines,
            } => html.push_str(&format!(
                "<pre><code = class=\"language-{language}\">{code_lines}</code></pre>"
            )),
            Node::Quote(t) => html.push_str(&format!("<q>{t}</q>")),
            Node::Table { headers, rows } => {
                html.push_str(&format!("<div class=\"table-wrapper\"><table><thead><tr>"));

                for row in headers {
                    for col in row {
                        html.push_str(&format!("<th>{col}</th>"));
                    }
                }

                html.push_str(&format!("</tr></thead>"));
                html.push_str(&format!("<tbody>"));

                for row in rows {
                    html.push_str(&format!("<tr>"));

                    for col in row {
                        html.push_str(&format!("<td>{col}</td>"));
                    }

                    html.push_str(&format!("</tr>"));
                }

                html.push_str(&format!("</tbody></table></div>"));
            }
        }
    }

    html
}

fn main() {
    let ast = parse(
        "# Product Update

        ## Highlights

        Ship notes are easier to publish when your draft stays in Markdown.

        - Faster editing for docs teams
        - Simple formatting for writers
        - Easy reuse inside CMS editors

        > Keep the structure clean before you paste the final HTML.

        ### Release Table

        | Area | Status |
        | --- | --- |
        | Docs | Ready |
        | Email | Drafting |

        ```js
        console.log('Markdown to HTML');
        ```

        Visit [the release page](https://markdowntoword.io/) for the full changelog.
        ",
    );

    let html = render(&ast);

    println!("{:?}", html);
}
