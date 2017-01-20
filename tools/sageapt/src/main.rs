use std::fs::File;
use std::io::BufReader;

extern crate clap;
use clap::{Arg, App, AppSettings, SubCommand};

extern crate libapt;
use libapt::{SwfHeader, ReadError};

fn main() {
    let matches = App::new("sageapt")
        .version("0.1.0")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .about("CLI for libapt")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("parse-header")
            .about("Parse the header of an APT file")
            .version("0.1.0")
            .author("Taryn Hill <taryn@phrohdoh.com>")
            .arg(Arg::with_name("apt_path")
                .value_name("apt_path")
                .required(true)
                .index(1)))
        .get_matches();

    let res: Result<SwfHeader, ReadError> = match matches.subcommand() {
        ("parse-header", Some(args)) => cmd_parse_header(args),
        _ => unreachable!(),
    };

    let code = if let Some(e) = res.err() {
        println!("CliError: {:?}", e);
        255
    } else {
        0
    };

    std::process::exit(code);
}

fn cmd_parse_header(args: &clap::ArgMatches) -> Result<SwfHeader, ReadError> {
    let path = args.value_of("apt_path").unwrap();
    let f = try!(File::open(&path));
    let mut br = BufReader::new(f);
    libapt::parse_header(&mut br)
}
