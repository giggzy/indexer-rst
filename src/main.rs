mod xml;

use std::{fs::File, io::Read};

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

    let pages = xml::deserialize_xml(contents);
    println!("Num. of Wiki Pages Deserialized: {:?}", pages.len());

    // transform in a format I can use

    // process to data into a index

    // search the index using a query

    // output the results

    // more
}
