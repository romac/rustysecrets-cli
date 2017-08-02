
// `error_chain` recursion adjustment
#![recursion_limit = "1024"]
// Make rustc's built-in lints more strict
#![warn(warnings)]

extern crate colored;
use colored::*;

extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

#[macro_use]
extern crate error_chain;

extern crate rusty_secrets;
use rusty_secrets::*;

#[macro_use]
mod verbose;
use verbose::*;

mod errors;
use errors::*;

mod validators;

use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

fn main() {
    if let Err(ref e) = run() {
        println!("{} {}", "    error:".red().bold(), e);

        for e in e.iter().skip(1) {
            println!("{} {}", "caused by:".yellow().bold(), e);
        }

        if let Some(backtrace) = e.backtrace() {
            println!("{} {:?}", "backtrace:".blue().bold(), backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let app =
        App::new("RustSecrets CLI")
            .version("0.1")
            .author("SpinResearch")
            .about("Split a secret of an arbitrary length in n different shares and k-out-of-n shares are required to recover it.")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .arg(Arg::with_name("verbose")
                 .short("v")
                 .long("verbose")
                 .help("Enable verbose mode"))
            .subcommand(SubCommand::with_name("split")
                        .about("Split a secret into shares")
                        .arg(Arg::with_name("k")
                             .short("k")
                             .required(true)
                             .takes_value(true)
                             .validator(validators::num::strictly_positive)
                             .help("Number of shares necessary to recover the secret"))
                        .arg(Arg::with_name("n")
                             .short("n")
                             .required(true)
                             .takes_value(true)
                             .validator(validators::num::strictly_positive)
                             .help("Total number of generated shares"))
                        .arg(Arg::with_name("DIR")
                             .short("o")
                             .long("output")
                             .required(true)
                             .takes_value(true)
                             .validator(validators::fs::directory)
                             .help("The directory to output the shares to"))
                        // .arg(Arg::with_name("MIME")
                        //      .short("m")
                        //      .long("mime")
                        //      .takes_value(true)
                        //      .validator(validators::mime)
                        //      .help("The MIME type of the secret"))
                        // .arg(Arg::with_name("sign")
                        //      .short("s")
                        //      .long("sign")
                        //      .help("Sign the shares"))
                        .arg(Arg::with_name("INPUT")
                             .required(true)
                             .validator(validators::fs::file)
                             .help("The file containing the secret to split")))
            .subcommand(SubCommand::with_name("recover")
                        .about("Recover the secret from the shares")
                        .arg(Arg::with_name("SHARES")
                             .required(true)
                             .takes_value(true)
                             .multiple(true)
                             .validator(validators::fs::file)
                             .help("The shares to recover the secret from"))
                        // .arg(Arg::with_name("MIME")
                        //      .short("m")
                        //      .long("mime")
                        //      .help("Print the MIME type of the secret on stderr"))
                        // .arg(Arg::with_name("verify")
                        //      .short("v")
                        //      .long("verify")
                        //      .help("Verify the shares signatures")));
                        .arg(Arg::with_name("FILE")
                             .short("o")
                             .long("output")
                             .takes_value(true)
                             .help("The file to output the secret to, printed on stdout otherwise")));

    let matches = app.get_matches();

    let verbose = matches.is_present("verbose");

    if let Some(matches) = matches.subcommand_matches("split") {
        let secret_path = Path::new(matches.value_of("INPUT").unwrap());
        let output_path = Path::new(matches.value_of("DIR").unwrap());
        let k = matches.value_of("k").unwrap().parse::<u8>().unwrap();
        let n = matches.value_of("n").unwrap().parse::<u8>().unwrap();
        let mime = matches.value_of("MIME");
        let sign = matches.is_present("sign");

        split(secret_path, output_path, k, n, mime, sign, verbose)?
    } else if let Some(matches) = matches.subcommand_matches("recover") {
        let shares = matches
            .values_of("SHARES")
            .unwrap()
            .map(Path::new)
            .collect();
        let output_path = matches.value_of("FILE").map(Path::new);

        recover(shares, output_path, verbose)?
    }

    Ok(())
}

fn split(
    secret_path: &Path,
    output_path: &Path,
    k: u8,
    n: u8,
    mime: Option<&str>,
    sign: bool,
    verbose: bool,
) -> Result<()> {
    if k > n {
        bail!("k must be smaller than or equal to n")
    }

    let mut secret = Vec::new();
    let mut secret_file = File::open(secret_path)
        .chain_err(|| "Could not open secret file")?;

    verbose!(verbose, "Reading secret {:?}... ", secret_path);

    let size = secret_file
        .read_to_end(&mut secret)
        .chain_err(|| "Could not read secret")?;

    verbose!(verbose, "Generating shares... ");

    let shares = generate_shares(k, n, &secret)
        .chain_err(|| "Could not generate shares")?;

    for (i, share) in shares.iter().enumerate() {
        let mut path_buf = output_path.to_path_buf();
        path_buf.push(format!("share_{}", i));
        let share_path = path_buf.as_path();

        verbose!(
            verbose,
            "Writing share #{} to {:?}...",
            i,
            share_path
        );

        let mut share_file = File::create(share_path)
            .chain_err(|| "Could not create share file")?;

        share_file
            .write_all(share.as_bytes())
            .chain_err(|| "Could not write share data to file")?;
    }

    Ok(())
}

fn recover(shares_paths: Vec<&Path>, output_path: Option<&Path>, verbose: bool) -> Result<()> {
    let mut shares = Vec::with_capacity(shares_paths.len());

    for share_path in shares_paths {
        if !share_path.exists() {
            bail!("Share {:?} does not exists", share_path);
        }
        if !share_path.is_file() {
            bail!("Share {:?} is not a file", share_path);
        }

        verbose!(verbose, "Reading share {:?}... ", share_path);

        let mut share_file = File::open(share_path).chain_err(|| format!("Could not open share {:?}", share_path))?;
        let mut share = String::new();
        let size = share_file.read_to_string(&mut share).chain_err(|| format!("Could not read share {:?}", share_path))?;
        shares.push(share);
    }

    verbose!(verbose, "Recovering secret... ");

    let secret = recover_secret(shares).chain_err(|| "Could not recover secret")?;

    match output_path {
        Some(output_path) => {
            let mut output_file = File::create(output_path).chain_err(|| format!("Could not create secret file {:?}", output_path))?;
            output_file.write_all(&secret).chain_err(|| format!("Could not write secret to file"))?;
        },
        None => {
            let secret_str = String::from_utf8(secret).chain_err(|| "Could not parse secret as UTF-8, consider outputting it to a file instead")?;
            println!("{}", secret_str);
        }
    }

    Ok(())
}

