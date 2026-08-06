use crate::ast::{Ast, Node};

#[derive(Debug)]
pub struct HtmlRenderer;

impl HtmlRenderer {
    pub fn render(ast: &Ast) -> String {
        let mut html = String::new();

        for item in ast.document.iter() {
            match item {
                Node::Heading { level, text } => {
                    html.push_str(&format!("<h{level}>{text}</h<{level}>"))
                }
                Node::Paragraph(t) => html.push_str(&format!("<p>{t}</p>")),
                Node::List { items } => {
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

                    for h in headers {
                        html.push_str(&format!("<th>{h}</th>"));
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
}
