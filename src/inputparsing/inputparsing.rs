#[derive(Debug)]
pub struct InputParsingErr {}

pub fn read_file_to_string(file_path: &str) -> Result<String, InputParsingErr> {
    std::fs::read_to_string(file_path).map_err(|_| InputParsingErr {})
}

pub trait OSMXMLParser<'a> {
    fn parse(&'a mut self, content: String) -> Result<(), InputParsingErr>;
    //fn traverse_tree(&'a self) -> Result<(), InputParsingErr>;
}

pub struct OSMXMLParserImpl<'a> {
    content: Option<String>,
    doc: Option<roxmltree::Document<'a>>,
}

/*
impl<'a> OSMXMLParserImpl<'a> {
    pub fn new() -> Self {
        Self {
            doc: None,
            content: None,
        }
    }
}  */

impl<'a> OSMXMLParserImpl<'a> {
    pub fn new() -> Self {
        Self {
            doc: None,
            content: None,
        }
    }
}

impl<'a> OSMXMLParser<'a> for OSMXMLParserImpl<'a> {
    fn parse(&'a mut self, content: String) -> Result<(), InputParsingErr> {
        self.content = Some(content.to_string());

        let content_ref: &String = self.content.as_ref().unwrap();
        let res = roxmltree::Document::parse(content_ref).map_err(|_| InputParsingErr {});

        match res {
            Ok(doc) => {
                self.doc = Some(doc);
                Ok(())
            }
            Err(InputParsingErr {}) => {
                self.doc = None;
                Err(InputParsingErr {})
            }
        }
    }

    /*
    fn traverse_tree(&'a self) -> Result<(), InputParsingErr> {
        Ok(())
    } */
}
