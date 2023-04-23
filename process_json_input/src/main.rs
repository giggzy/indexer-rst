use quick_xml::de::from_str as from_str_xml;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

use std::cmp::min;

use serde_derive::{Deserialize, Serialize};
use serde_xml_rs::from_str;
use std::path::Path;
use std::{
    fs::File,
    io::{self, BufRead, Read},
};

//const DATADIR: &str = "data";

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

fn read_some_lines(filename: String, linecount: usize) {
    for (line_num, line) in read_lines(filename).enumerate() {
        println!("{}", line.unwrap());
        if line_num > linecount {
            break;
        }
    }
}

// create a function to read in the XML file
fn read_xml_file(filename: String) {
    let mut reader = Reader::from_file(filename).unwrap();

    let mut count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    loop {
        if count > 100 {
            break;
        }
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"tag1" => println!(
                    "attributes values: {:?}",
                    e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                ),
                b"tag2" => count += 1,
                _ => (),
            },
            Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        buf.clear();
        count += 1;
    }
    println!("Done");
    print!("Text: {:?}", txt);
}

// Struct for the Wikipedia XML file

#[derive(Debug, Deserialize, Serialize)]
struct Mediawiki {
    siteinfo: SiteInfo,
    page: Vec<Page>,
}
/*
impl std::str::FromStr for Document {
    type Err = serde_xml_rs::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_xml_rs::from_str(s)
    }
}
 */
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
struct Page {
    title: String,
    //ns: String,
    id: String,
    revision: Revision,
}

#[derive(Debug, Deserialize, Serialize)]
struct Revision {
    id: String,
    parentid: Option<String>,
    timestamp: String,
    contributor: Contributor,
    model: Option<String>,
    format: Option<String>,
    text: String,
    sha1: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Contributor {
    username: Option<String>,
    id: Option<String>,
}

fn serialize_xml_file(filename: String) {
    // time the steps
    let start = std::time::Instant::now();

    // read file into string
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!(
        "Read file into memory: {} seconds",
        start.elapsed().as_secs()
    );
    println!("Size: {}", contents.len());

    // print out part of string
    println!("Contents: {}", &contents[0..1000]);

    //let doc: Document = from_str(&contents).unwrap();
    let doc: Mediawiki = from_str(&contents).unwrap();
    println!("Deserialize: {} seconds", start.elapsed().as_secs());
    println!("Deserialize: {} minutes", start.elapsed().as_secs() / 60);

    //println!("Document: {:?}", doc);
    //println!("Page: {:?}", doc.page);
    println!("Page Len: {:?}", doc.page.len());
}

fn serialize_xml_file2(filename: String) {
    // Try and use a streaming parser

    // time the steps
    let start = std::time::Instant::now();

    // read file into string
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!(
        "Read file into memory: {} seconds",
        start.elapsed().as_secs()
    );
    println!("Size: {}", contents.len());

    // print out part of string
    println!("Contents: {}", &contents[0..1000]);

    let mut reader = Reader::from_str(&contents);
    //let mut txt = Vec::new();
    let mut count = 0;
    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => match e.name().as_ref() {
                b"page" => count += 1,
                b"revision" => {
                    count += 1;
                }
                _ => (),
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
    //let doc: Document = from_str(&contents).unwrap();
    println!("Deserialize: {} seconds", start.elapsed().as_secs());
    println!("Deserialize: {} minutes", start.elapsed().as_secs() / 60);

    //println!("Document: {:?}", doc);
    //println!("Page: {:?}", doc.page);
    println!("Page Len: {:?}", count);
}

fn deserialize_xml(contents: String) -> Vec<Page> {
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
            page.revision.text,
            page.revision.text.len(),
        );
    }
    object.page
}
use rusqlite::{Connection, Result};

struct DB {
    is_initialized: bool,
    conn: Connection,
}

impl DB {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    fn new(Self) -> Result<DB> {
        if self.is_initialized() {
            return Ok(self);
        }

        let conn = Connection::open_in_memory()?;
        conn.execute(
            "CREATE TABLE page (
                  id              INTEGER PRIMARY KEY,
                  title           TEXT NOT NULL,
                  revision_id     INTEGER NOT NULL,
                  revision_text   TEXT NOT NULL,
                  revision_text_length INTEGER NOT NULL
                  )",
            (),
        )?;
        Ok(DB {
            conn: conn,
            is_initialized: true,
        })
    }
}
// const DB_NAME: &str = "wiki.db";
// fn db_init() -> Result<()> {
//     let is_db_initialized: bool = false;
//     let conn = Connection::open_in_memory()?;
//     conn.execute(
//         "CREATE TABLE page (
//                   id              INTEGER PRIMARY KEY,
//                   title           TEXT NOT NULL,
//                   revision_id     INTEGER NOT NULL,
//                   revision_text   TEXT NOT NULL,
//                   revision_text_length INTEGER NOT NULL
//                   )",
//         (),
//     )?;
//     Ok(())
// }
fn save_page_to_db(page: Page) {
    // save the page to the database
    let db = DB::new();
    let conn = db.conn;
    conn.execute(
        "INSERT INTO page (title, revision_id, revision_text, revision_text_length)
                  VALUES (?1, ?2, ?3, ?4)",
        params![
            page.title,
            page.revision.id,
            page.revision.text,
            page.revision.text.len()
        ],
    )?;
}

fn main() {
    println!("Hello, world!");

    // panic if no filename is passed
    if std::env::args().len() < 2 {
        panic!("No filename passed");
    }

    // get the file name from the command line
    let filename = std::env::args().nth(1).unwrap();
    println!("The filename is: {}", filename);

    // read file into string
    let mut file = File::open(filename.clone()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    //let linecount: usize = std::env::args().nth(2).unwrap().parse().unwrap_or(10);
    //read_some_lines(filename, linecount);

    //read_xml_file(filename);
    serialize_xml_file2(filename.clone());
    //serialize_xml_file(filename);
    deserialize_xml(contents);

    // transform in a format I can use

    // process to data into a index

    // search the index using a query

    // output the results

    // more
}
