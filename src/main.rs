mod db;
mod xml;

use clap::{Parser, Subcommand};
use std::{dbg, fs::File, io::Read};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Index a file")]
    Index {
        #[arg(short, long)]
        filename: String,
    },
    // #[command(about = "Search an index")]
    // Search(Search),
}

fn main() {
    let args = Args::parse();
    println!("The Args are: {:?}", args);
    println!("Hello, world!");

    if let Some(Commands::Index { filename }) = args.command {
        println!("Indexing file: {}", filename);
        let mut file = File::open(filename).expect("Something went wrong opening the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Something went wrong reading the file");

        let pages = xml::deserialize_xml(contents);
        println!("Num. of Wiki Pages Deserialized: {:?}", pages.len());

        // save each page to sqlite DB
        for page in pages {
            db::insert_page(&page);
            // tokenize on whitespace
            let tokens: Vec<&str> = page.revision.text.split_whitespace().collect();
            println!("Page: {}, Tokens count: {}", page.title, tokens.len());
        }

        let my_page = db::get_page(&"AlgeriA").unwrap();
        dbg!(my_page.revision_text);
    }
    // read file into string
    // let mut file = File::open(args.filename.clone()).unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)
    //     .expect("Something went wrong reading the file");

    // let pages = xml::deserialize_xml(contents);
    // println!("Num. of Wiki Pages Deserialized: {:?}", pages.len());

    // transform in a format I can use

    // process to data into a index

    // search the index using a query

    // output the results

    // more
}
