use pulldown_cmark::{html::push_html, Options, Parser};
const CSS: &str = include_str!("../assets/github.min.css");
const JS: &str = include_str!("../assets/highlight.min.js");

pub fn to_html(input: &str) -> String {
    let parser = Parser::new_ext(input, Options::all());

    let mut html_content = String::new();
    push_html(&mut html_content, parser);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<title>GTK Markdown Preview</title>
<style>{css}</style>
<script>{js}</script>
<script>hljs.highlightAll();</script>
<style>
    body {{ font-family: sans-serif; margin: 1rem; }}
    pre {{ padding: 0.5rem; border-radius: 5px; }}
    pre > code {{ display: block; }}
</style>
</head>
<body>
{html}
</body>
</html>"#,
        css = CSS,
        js = JS,
        html = html_content
    )
}
