// some integrations tests for the xml module

// import the module we want to test

// import the xml module
use indexer::xml::deserialize_xml;
use std::{fs::File, io::Read};

// use the test crate
#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_xml(filename: String) -> String {
        // read file into string
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        return contents;
    }
    #[test]
    fn test_deserialize_xml() {
        // read file into string
        // let mut file = File::open("data/test.xml").unwrap();
        // let mut contents = String::new();
        // file.read_to_string(&mut contents).unwrap();
        let contents = get_test_xml("data/test.xml".to_string());

        let pages = deserialize_xml(contents);
        assert_eq!(pages.len(), 2);
        assert_eq!(pages[0].title, "AaA");
        assert_eq!(pages[0].revision.id, "32899315");
        assert_eq!(pages[0].revision.get_text_short(), "#REDIRECT [[AAA]]");
        assert_eq!(pages[1].title, "AlgeriA");
        assert_eq!(pages[1].revision.id, "18063769");
        assert_eq!(
            pages[1].revision.get_text_short(),
            "#REDIRECT [[Algeria]]{{R from CamelCase}}"
        );
    }

    #[test]
    #[should_panic]
    fn test_deserialize_xml_misformed_file() {
        let contents = get_test_xml("data/test_bad.xml".to_string());
        let pages = deserialize_xml(contents);
        assert_eq!(pages.len(), 2);
    }
}
