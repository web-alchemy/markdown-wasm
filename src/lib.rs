use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(source: String) -> String {
    let parser = pulldown_cmark::Parser::new(&source);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    return html_output;
}
