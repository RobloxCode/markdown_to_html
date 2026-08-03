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
    content.lines().collect()
}
