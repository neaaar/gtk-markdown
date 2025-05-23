use pulldown_cmark::{html::push_html, Options, Parser};

pub fn to_html(input: &str) -> String {
    let options = Options::all();
    let parser = Parser::new_ext(input, options);

    let mut html = String::new();
    push_html(&mut html, parser);
    html
}
