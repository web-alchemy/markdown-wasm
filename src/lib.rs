use pulldown_cmark::{Options, Parser};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[allow(non_snake_case)]
pub struct Flags {
    pub ENABLE_TABLES:                           u32,
    pub ENABLE_FOOTNOTES:                        u32,
    pub ENABLE_STRIKETHROUGH:                    u32,
    pub ENABLE_TASKLISTS:                        u32,
    pub ENABLE_SMART_PUNCTUATION:                u32,
    pub ENABLE_HEADING_ATTRIBUTES:               u32,
    pub ENABLE_YAML_STYLE_METADATA_BLOCKS:       u32,
    pub ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS: u32,
    pub ENABLE_OLD_FOOTNOTES:                    u32,
    pub ENABLE_MATH:                             u32,
    pub ENABLE_GFM:                              u32,
    pub ENABLE_DEFINITION_LIST:                  u32,
}

#[wasm_bindgen]
impl Flags {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Flags {
        Flags {
            ENABLE_TABLES:                           Options::ENABLE_TABLES.bits(),
            ENABLE_FOOTNOTES:                        Options::ENABLE_FOOTNOTES.bits(),
            ENABLE_STRIKETHROUGH:                    Options::ENABLE_STRIKETHROUGH.bits(),
            ENABLE_TASKLISTS:                        Options::ENABLE_TASKLISTS.bits(),
            ENABLE_SMART_PUNCTUATION:                Options::ENABLE_SMART_PUNCTUATION.bits(),
            ENABLE_HEADING_ATTRIBUTES:               Options::ENABLE_HEADING_ATTRIBUTES.bits(),
            ENABLE_YAML_STYLE_METADATA_BLOCKS:       Options::ENABLE_YAML_STYLE_METADATA_BLOCKS.bits(),
            ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS: Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS.bits(),
            ENABLE_OLD_FOOTNOTES:                    Options::ENABLE_OLD_FOOTNOTES.bits(),
            ENABLE_MATH:                             Options::ENABLE_MATH.bits(),
            ENABLE_GFM:                              Options::ENABLE_GFM.bits(),
            ENABLE_DEFINITION_LIST:                  Options::ENABLE_DEFINITION_LIST.bits(),
        }
    }
}

#[wasm_bindgen]
pub fn parse(source: &str, flags: Option<u32>) -> String {
    let options = match flags {
        Some(value) => Options::from_bits(value).unwrap(),
        None => Options::empty()
    };
    let parser = Parser::new_ext(source, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    return html_output;
}
