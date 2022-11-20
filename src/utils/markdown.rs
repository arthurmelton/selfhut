use emojicons::EmojiFormatter;
use pulldown_cmark::{html, Options, Parser};

pub fn md_to_html(input: &str) -> String {
    let input = EmojiFormatter(input).to_string();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
