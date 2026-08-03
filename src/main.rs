use std::fs;

fn main() {
    let path = "src/md_src.md";
    let content = match read_file(path) {
        Ok(content) => content,
        Err(e) => panic!("couldn't open {}: {}", path, e),
    };

    let lines = to_lines(&content);

    let html = to_html(&lines);

    println!("{}", html);

    fs::write("content.html", html).unwrap();
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

fn to_lines(content: &str) -> Vec<&str> {
    content.lines().collect()
}

fn to_html(lines: &[&str]) -> String {
    let mut parsed = String::new();

    for line in lines {
        if let Some(first_char) = line.chars().nth(0) {
            match first_char {
                '#' => {
                    parsed.push_str("<h1>");
                    parsed.push_str(&line[1..]);
                    parsed.push_str("</h1>\n");
                }

                '-' => {
                    parsed.push_str("<ul>");
                    parsed.push_str("<li>");
                    parsed.push_str(&line[1..]);
                    parsed.push_str("</li>");
                    parsed.push_str("</ul>\n");
                }

                '>' => {
                    parsed.push_str("<blokcquote>\n");
                    parsed.push_str("<p>");
                    parsed.push_str(&line[1..]);
                    parsed.push_str("</p>\n");
                    parsed.push_str("</blokcquote>\n");
                }

                '|' => {
                    parsed.push_str("<table>\n");
                    parsed.push_str("</table>\n");
                }

                _ => {
                    parsed.push_str("<p>");
                    parsed.push_str(&line[1..]);
                    parsed.push_str("</p>\n");
                }
            }
        }
    }

    parsed
}
