use ammonia;
use pulldown_cmark as md;

type ParseFunction = dyn Fn(&str, ParseOptions) -> String;

pub fn parse_content(
    content: &str,
    options: ParseOptions,
    parse_function: Box<ParseFunction>,
    escape_html: bool,
) -> String {
    if escape_html {
        unimplemented!();
    }
    let parsed = parse_function(content, options);
    ammonia::clean(&parsed)
}

pub fn parse_markdown(content: &str, parse_options: ParseOptions) -> String {
    let mut options = md::Options::empty();
    match parse_options {
        ParseOptions::All => {
            options = md::Options::all();
        }
        _ => {}
    }
    let parser = md::Parser::new_ext(content, options);
    let mut html_output = String::new();
    md::html::push_html(&mut html_output, parser);
    html_output
}

pub enum ParseOptions {
    All,
    List(Vec<ParseOptionValue>),
    None,
}

pub enum ParseOptionValue {
    ENABLE_TABLES,
    ENABLE_FOOTNOTES,
    ENABLE_STRIKETHROUGH,
    ENABLE_TASKLISTS,
    ENABLE_SMART_PUNCTUATION,
    ENABLE_HEADING_ATTRIBUTES,
}

impl From<ParseOptionValue> for md::Options {
    fn from(p: ParseOptionValue) -> md::Options {
        match p {
            ParseOptionValue::ENABLE_TABLES => md::Options::ENABLE_TABLES,
            ParseOptionValue::ENABLE_FOOTNOTES => md::Options::ENABLE_FOOTNOTES,
            ParseOptionValue::ENABLE_STRIKETHROUGH => md::Options::ENABLE_STRIKETHROUGH,
            ParseOptionValue::ENABLE_TASKLISTS => md::Options::ENABLE_TASKLISTS,
            ParseOptionValue::ENABLE_SMART_PUNCTUATION => md::Options::ENABLE_SMART_PUNCTUATION,
            ParseOptionValue::ENABLE_HEADING_ATTRIBUTES => md::Options::ENABLE_HEADING_ATTRIBUTES,
        }
    }
}
