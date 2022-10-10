use clap::Parser;
use std::fs::File;
use std::path::PathBuf;
use std::io::Read;
use std::io::Write;
use std::io::stdin;

mod lib;

#[derive(Parser)]
#[command(name = "swf-utils")]
#[command(author = "creyon <gtcreyongmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Command line tools to manipulate Adobe Flash SWF files and standalone Flash executables.", long_about = None)]
struct Arguments {
    /// Input file path. Omitting will read from stdin.
    #[arg(short='i', long="input")]
    path_in: Option<PathBuf>,
    /// Output file path. Omitting will print to stdout.
    #[arg(short='o', long="output")]
    path_out: Option<PathBuf>,
    /// File path to a standalone projector executable. If specified, will create a bundle.
    #[arg(short='b', long="bundlewith")]
    path_bundle: Option<PathBuf>,
    /// Retrieve the standalone executable from the bundle, instead of the SWF.
    #[arg(short='e', long="executable")]
    get_executable: bool,
}

fn main() {
    // Parse CLI arguments
    let args = Arguments::parse();

    // Contains either SWF or EXE data
    let mut raw_data = vec![];
    
    // Data input
    match args.path_in {
        // Input path is specified, so use that file
        Some(path) => {
            let mut file_in = File::open(path).unwrap_or_else(|error| {
                panic!("Problem opening the file: {:?}", error);
            });
            file_in.read_to_end(&mut raw_data).unwrap_or_else(|error| {
                panic!("Problem reading the contents of the file: {:?}", error);
            });
        },
        // Input path is omitted, so use stdin
        None => {
            stdin().read_to_end(&mut raw_data).unwrap_or_else(|error| {
                panic!("Problem reading the contents of stdin: {:?}", error);
            });
        },
    };
    
    // Data processing
    let output_data = match args.path_bundle {
        // Bundle path is specified, so bundle SWF with it
        Some(path) => {
            let mut file_bundle = File::open(path).unwrap_or_else(|error| {
                panic!("Problem opening the file: {:?}", error);
            });
            let mut bundle = vec![];
            file_bundle.read_to_end(&mut bundle).unwrap_or_else(|error| {
                panic!("Problem reading the contents of the file: {:?}", error);
            });
            lib::bundle_swf_exe_bytes(&mut raw_data, &mut bundle)
        },
        // Bundle path is omitted, so extract as normal
        None => {
            let size = lib::swf_size_from_buf(&raw_data).expect("File is not a valid SWF bundle.");
            if args.get_executable {
                lib::exe_bytes_from_buf(&raw_data, &size)
            } else {
                lib::swf_bytes_from_buf(&raw_data, &size)
            }
        },
    };
    
    // Data output
    match args.path_out {
        // Output path is specified, so save to that file
        Some(path) => {
            let mut file_out = File::create(path).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            });
            file_out.write(&output_data).unwrap_or_else(|error| {
                panic!("Problem writing to the file: {:?}", error);
            });
        },
        // Output path is omitted, so print to stdout
        None => {
            println!("{:?}", &output_data);
        },
    };
}
