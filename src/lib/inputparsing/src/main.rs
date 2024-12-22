use std::env;
use std::io::{self, Write};
use std::time::SystemTime;

use wmaps_algorithms::algorithms;
use wmaps_algorithms::datastructures;

use inputparsing::OSMXMLParser;

pub mod inputparsing;

macro_rules! print_now {
    ($s: literal) => {
        print!("{}", $s);
        io::stdout()
            .flush()
            .expect("Error flushing stdout in print_now macro");
    };
}

macro_rules! print_time_ms {
    ($timer: expr) => {
        println!(
            " {}ms",
            $timer
                .elapsed()
                .expect("Something went wrong with timing the execution.")
                .as_millis()
        );
    };
}

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

    let mut now = SystemTime::now();
    print_now!("Read input file.....");

    let file_content: String =
        inputparsing::read_file_to_string(input_path).expect("Error reading the input file");

    print_time_ms!(now);

    now = SystemTime::now();

    print_now!("Build xml tree.....");

    let mut parser = inputparsing::OSMXMLParserImpl::new();

    parser
        .parse(file_content)
        .expect("Error parsing the input file");

    print_time_ms!(now);

    now = SystemTime::now();

    print!("Traverse xml tree.....");

    println!("Moin");
    datastructures::test();
    algorithms::test();
}
