use quick_xml::de::from_str as from_str_xml;
use serde_derive::{Deserialize, Serialize};
use std::cmp::min;

// Structs for the Wikipedia XML file
#[derive(Debug, Deserialize, Serialize)]
struct Mediawiki {
    siteinfo: SiteInfo,
    page: Vec<Page>,
}
#[derive(Debug, Deserialize, Serialize)]
struct SiteInfo {
    sitename: String,
    base: String,
    generator: String,
    case: String,
    namespaces: Namespaces,
}

#[derive(Debug, Deserialize, Serialize)]
struct Namespaces {
    namespace: Vec<Namespace>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Namespace {
    key: Option<String>,
    value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    pub title: String,
    //ns: String,
    pub id: String,
    pub revision: Revision,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Revision {
    pub id: String,
    parentid: Option<String>,
    timestamp: String,
    contributor: Contributor,
    model: Option<String>,
    format: Option<String>,
    text: String,
    sha1: Option<String>,
}

impl Revision {
    pub fn get_text_short(&self) -> String {
        let max_to_show = min(100, self.text.len());
        if max_to_show < self.text.len() {
            return self.text[0..max_to_show].to_string() + "...";
        }
        return self.text.to_string();
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Contributor {
    username: Option<String>,
    id: Option<String>,
}

// FUNCTIONS

pub fn deserialize_xml(contents: String) -> Vec<Page> {
    // Try and use a streaming parser to deserialize into a struct

    // time the steps
    let start = std::time::Instant::now();

    let object: Mediawiki = from_str_xml(&contents).unwrap();
    println!("Deserialize: {} seconds", start.elapsed().as_secs());
    println!("Deserialize: {} minutes", start.elapsed().as_secs() / 60);
    // print some of the titles
    let max_to_show = min(object.page.len(), 100);
    for page in object.page[0..max_to_show].iter() {
        println!(
            "Title: {:?}, Content: {:?}, Length: {:?} ",
            page.title,
            page.revision.get_text_short(),
            page.revision.text.len(),
        );
    }
    object.page
}
