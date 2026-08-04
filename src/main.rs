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
        } else {
            document.push(Node::Paragraph(trimmed.to_string()));
        }
    }

    Ast { document }
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

    println!("{:#?}", ast);
}
