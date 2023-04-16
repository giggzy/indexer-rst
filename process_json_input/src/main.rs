use std::fs::File;

fn main() {
    println!("Hello, world!");
    
    // read in json file
    let file = File::open("data.json").unwrap();  

    let contents =  file.read_to_string(&mut contents).unwrap();
    println("The file contents are: {}", contents);
    // transform in a format I can use

    // process to data into a index

    // search the index using a query

    // output the results
}
