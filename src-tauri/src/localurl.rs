
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
        Some(".png") => "image/x-png",
        Some(".jpg") | Some(".jpeg") => "image/jpg",
        Some(".svg") =>  "image/svg+xml",
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