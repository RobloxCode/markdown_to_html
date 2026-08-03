use std::fs;

struct Parser {
    path: &'static str,
    fcontent: String,
}

impl Parser {
    fn new(path: &'static str) -> Self {
        Self {
            path,
            fcontent: String::new(),
        }
    }

    fn to_html(&mut self) -> String {
        self.fcontent = match read_file(self.path) {
            Ok(content) => content,
            Err(e) => panic!("couldn't open {}: {}", self.path, e),
        };

        let lines = to_lines(&self.fcontent);
        let mut parsed = String::new();

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];

            let first_char = line.chars().nth(0).unwrap_or_default();
            let sec_char = line.chars().nth(1).unwrap_or_default();
            let third_char = line.chars().nth(2).unwrap_or_default();

            if first_char == '#' && sec_char == '#' && third_char == '#' {
                parsed.push_str("<h3>");
                parsed.push_str(&line[4..]);
                parsed.push_str("</h3>\n");
                i += 1;
                continue;
            } else if first_char == '#' && sec_char == '#' {
                parsed.push_str("<h2>");
                parsed.push_str(&line[3..]);
                parsed.push_str("</h2>\n");
                i += 1;
                continue;
            } else if first_char == '#' {
                parsed.push_str("<h1>");
                parsed.push_str(&line[2..]);
                parsed.push_str("</h1>\n");
                i += 1;
                continue;
            }

            if first_char == '`' && sec_char == '`' && third_char == '`' {
                parsed.push_str("<pre><code class=\"language-js\">");
                parsed.push_str(&lines[i + 1]);
                parsed.push_str("</code></pre>\n");
                i += 3;
                continue;
            }

            match first_char {
                '-' => {
                    parsed.push_str("<ul>");
                    parsed.push_str("<li>");
                    parsed.push_str(&line[1..]);
                    parsed.push_str("</li>");
                    parsed.push_str("</ul>\n");
                    i += 1;
                }

                '>' => {
                    parsed.push_str("<blockquote>\n");
                    parsed.push_str("<p>");
                    parsed.push_str(&line[1..]);
                    parsed.push_str("</p>\n");
                    parsed.push_str("</blockquote>\n");
                    i += 1;
                }

                '|' => {
                    parsed.push_str("<table>\n");
                    parsed.push_str("</table>\n");
                    i += 1;
                }

                _ => {
                    parsed.push_str("<p>");
                    parsed.push_str(&line[1..]);
                    parsed.push_str("</p>\n");
                    i += 1;
                }
            }
        }

        fs::write("content.html", parsed.clone()).unwrap();

        parsed
    }
}

fn main() {
    let mut p = Parser::new("src/md_src.md");
    let html = p.to_html();
    println!("{}", html);
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

fn to_lines(content: &str) -> Vec<&str> {
    content.lines().filter(|&c| !c.is_empty()).collect()
}
