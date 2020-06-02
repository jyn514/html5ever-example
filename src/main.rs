use failure::err_msg;
use kuchiki::traits::TendrilSink;
use kuchiki::NodeRef;

type Result<T> = std::result::Result<T, failure::Error>;

fn main() {
    let regex = include_str!("struct.CaptureMatches.html");
    loop {
        let (_head, _body, _classes) = extract_head_and_body(regex).unwrap();
    }
}


/// Extracts the contents of the `<head>` and `<body>` tags from an HTML document, as well as the
/// classes on the `<body>` tag, if any.
pub fn extract_head_and_body(html: &str) -> Result<(String, String, String)> {
    let dom = kuchiki::parse_html().one(html);

    let head = dom.select_first("head").map_err(|_| err_msg("couldn't find <head> tag in rustdoc output"))?;
    let body = dom.select_first("body").map_err(|_| err_msg("couldn't find <body> tag in rustdoc output"))?;

    let class = body.attributes.borrow().get("class").map(|v| v.to_owned()).unwrap_or_default();

    Ok((serialize(head.as_node()), serialize(body.as_node()), class))
}

fn serialize(v: &NodeRef) -> String {
    let mut contents = Vec::new();
    for child in v.children() {
        child.serialize(&mut contents).expect("serialization failed");
    }
    String::from_utf8(contents).expect("non utf-8 html")
}
