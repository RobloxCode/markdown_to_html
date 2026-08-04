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

        if trimmed.starts_with('#') {
            let level = trimmed.chars().take_while(|&c| c == '#').count();
            let text = trimmed[level..].to_string();

            document.push(Node::Heading { level, text });
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
