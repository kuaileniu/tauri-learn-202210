extern crate regex;
extern crate rustc_serialize;

use regex::Regex;
use rustc_serialize::base64::{FromBase64, ToBase64, MIME};
use rustc_serialize::hex::ToHex;
use std::fs::File;
use std::io::Read;
use std::string::String;

#[no_mangle]
pub fn get_image_filetype(header: &str) -> Option<&str> {
    let mut result = None;
    if Regex::new(r"^89504e47").unwrap().is_match(header) {
        result = Some("png");
    } else if Regex::new(r"^ffd8ffe0").unwrap().is_match(header) {
        result = Some("jpg");
    } else if Regex::new(r"^47494638").unwrap().is_match(header) {
        result = Some("gif");
    }
    result
}

#[no_mangle]
pub fn image_encode_base64(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);
    let base64 = vec.to_base64(MIME);
    let hex = vec.to_hex();
    return format!(
        "data:image/{};base64,{}",
        get_image_filetype(&hex).unwrap(),
        base64.replace("\r\n", "")
    );
}

#[no_mangle]
pub fn base64_to_vec(base64: String) -> Vec<u8> {
    let offset = base64.find(',').unwrap_or(base64.len()) + 1;
    let mut value = base64;
    value.drain(..offset);
    return value.from_base64().unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
