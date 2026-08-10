# markdown to html

A tiny Markdown-to-HTML converter written in Rust, no external crates.

## Features

- Headers: `#`, `##`, `###`
- Unordered lists: `- item`
- Paragraphs

Intentionally minimal — no nested lists, code blocks, links, or tables.

## Build

```bash
cargo build --release
```

## Usage
you can specify a path or else it would use the code in the file `src/input.md`
```bash
cargo run [path]
```

Or, using the compiled binary:

```bash
./target/release/md2html input.md > output.html
```

## Example

**input.md**
```markdown
# Hello World

This is a **bold** statement and this is *italic*.

- First item
- Second item
```

**output.html**
```html
<h1>Hello World</h1>
<p>This is a <strong>bold</strong> statement and this is <em>italic</em>.</p>
<ul>
<li>First item</li>
<li>Second item</li>
</ul>
```

## How it works

The converter does two passes:

1. **Line-level parsing** — walks the file line by line, detecting block-level elements (headers, list items, paragraphs) based on line prefixes.
2. **Inline parsing** — within each line's text, scans character by character for `**` and `*` delimiters to wrap matched spans in `<strong>` or `<em>`.

## Possible extensions

- Ordered lists (`1.`, `2.`, ...)
- Code blocks (```` ``` ````) and inline code (`` `code` ``)
- Links (`[text](url)`) and images (`![alt](url)`)
- Nested lists
- Blockquotes (`>`)
