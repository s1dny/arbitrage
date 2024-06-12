use ammonia::Builder;
use maplit::hashset;
use sanitize_html::sanitize_str;
use sanitize_html::rules::predefined::DEFAULT;
use std::fs;

fn main() {
    let file = match fs::read_to_string("website.html") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let text: String = sanitize_str(&DEFAULT, &file).unwrap();

    let html = Builder::default()
        .link_rel(None)
        .rm_tags(&["noscript"])
        .tags(hashset!["p", "li", "ul", "a", "h2", "span"])
        .clean(&file)
        .to_string()
        .replace("&nbsp;", "");

    println!("{}", text);

    println!("{}", html);
}