
use crate::noteerror::{NoteResult};
use urlencoding::decode;
use regex::Regex;
use std::path::Path;
use std::ffi::OsStr;

#[derive(Debug)]
pub struct LocalUrl {
    pub note: String,
    pub filename: String,
    pub mime_type: String,
}

fn get_mimetype(filename: &str) -> String {
    let extension = Path::new(filename).extension().and_then(OsStr::to_str);

    let mime_type = match extension {
        Some("png") => "image/x-png",
        Some("jpg") | Some("jpeg") => "image/jpg",
        Some("svg") =>  "image/svg+xml",
        _ => "application/octet-stream"
    };

    String::from(mime_type)
}

impl LocalUrl {
    pub fn parse(url: &str) -> NoteResult<Self> {
        let re = Regex::new(r"local://notes/([^/]+)/([^/?#]+).*").unwrap();
        let Some(caps) = re.captures(url) else {
            return Err("invalid url".into());
        };

        let note = decode(&caps[1])?.to_string();
        let filename = decode(&caps[2])?.to_string();
        let mime_type = get_mimetype(&filename);

        Ok(LocalUrl {
            note: note,
            filename: filename,
            mime_type: mime_type,
        })
    } 
}

#[cfg(test)]
mod tests {

use crate::localurl::get_mimetype;
use crate::localurl::LocalUrl;

#[test]
fn get_some_mimetype() {
    assert_eq!("image/x-png", get_mimetype("some.png"));
    assert_eq!("image/jpg", get_mimetype("some.jpg"));
    assert_eq!("image/jpg", get_mimetype("some.jpeg"));
    assert_eq!("image/svg+xml", get_mimetype("some.svg"));

    assert_eq!("application/octet-stream", get_mimetype("some.bin"));
}

#[test]
fn parse_url() {
    let result = LocalUrl::parse("local://notes/my_note/my.png");
    assert!(result.is_ok());

    let url = result.unwrap();
    assert_eq!("my_note", url.note);
    assert_eq!("my.png", url.filename);
    assert_eq!("image/x-png", url.mime_type);
}

#[test]
fn parse_url_ignore_query() {
    let result = LocalUrl::parse("local://notes/my_note/my.png?some_query");
    assert!(result.is_ok());

    let url = result.unwrap();
    assert_eq!("my_note", url.note);
    assert_eq!("my.png", url.filename);
    assert_eq!("image/x-png", url.mime_type);
}

#[test]
fn parse_url_ignore_fragment() {
    let result = LocalUrl::parse("local://notes/my_note/my.png#fragment");
    assert!(result.is_ok());

    let url = result.unwrap();
    assert_eq!("my_note", url.note);
    assert_eq!("my.png", url.filename);
    assert_eq!("image/x-png", url.mime_type);
}

#[test]
fn parse_encoded_url() {
    let result = LocalUrl::parse("local://notes/my%20note/my%20pic.jpg");
    assert!(result.is_ok());

    let url = result.unwrap();
    assert_eq!("my note", url.note);
    assert_eq!("my pic.jpg", url.filename);
    assert_eq!("image/jpg", url.mime_type);
}

#[test]
fn fail_to_parse_invalid_scheme() {
    let result = LocalUrl::parse("file://notes/note/pic.jpg");
    assert!(result.is_err());
}

#[test]
fn fail_to_parse_missing_notes() {
    let result = LocalUrl::parse("local://note/pic.jpg");
    assert!(result.is_err());
}

#[test]
fn fail_to_parse_missing_note() {
    let result = LocalUrl::parse("local://notes//pic.jpg");
    assert!(result.is_err());
}

#[test]
fn fail_to_parse_missing_file() {
    let result = LocalUrl::parse("local://notes/note");
    assert!(result.is_err());
}


}