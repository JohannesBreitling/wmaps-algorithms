use std::env;
use std::time::SystemTime;

use inputparsing::OSMXMLParser;

mod inputparsing;

fn main() {
    println!("BUILDING INPUTS...");
    println!("- - - - - - - - - - - - - - -");

    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        panic!("No input file path provided");
    }

    if args.len() > 2 {
        panic!("Please only provide one input file path");
    }

    let input_path: &str = &args[1];

    let now = SystemTime::now();
    print!("Read input file.....");
    let file_content: String =
        inputparsing::read_file_to_string(input_path).expect("Error reading the input file");
    let time_file_reading = now.elapsed();

    let now = SystemTime::now();

    println!(
        " {}ms",
        time_file_reading
            .expect("Something went wrong with timing the execution.")
            .as_millis()
    );

    print!("Build xml tree.....");

    let mut parser = inputparsing::OSMXMLParserImpl::new();
    parser
        .parse(file_content)
        .expect("Error parsing the input file");

    let time_parse_xml = now.elapsed();

    println!(
        " {}ms",
        time_parse_xml
            .expect("Something went wrong with timing the execution.")
            .as_millis()
    );

    print!("Traverse xml tree.....");
}
