
// `error_chain` recursion adjustment
#![recursion_limit = "1024"]

// Make rustc's built-in lints more strict
#![warn(warnings)]

extern crate colored;
use colored::*;

extern crate clap;

#[macro_use]
extern crate error_chain;

extern crate rusty_secrets;
use rusty_secrets::*;

#[macro_use]
mod verbose;

mod errors;
use errors::*;

mod input;
use input::Input;

mod app;
use app::app;

mod validators;

use std::path::Path;
use std::fs::File;
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
    let matches = app().get_matches();

    let verbose = matches.is_present("verbose");

    if let Some(matches) = matches.subcommand_matches("split") {
        let secret_arg = matches.value_of("INPUT").unwrap();
        let secret_input = if secret_arg == "-" {
            Input::stdin()
        } else {
            Input::file(secret_arg.to_string())
                .chain_err(|| format!("Could not open secret file \"{}\"", secret_arg))?
        };

        let output_path = Path::new(matches.value_of("DIR").unwrap());
        let k = matches.value_of("k").unwrap().parse::<u8>().unwrap();
        let n = matches.value_of("n").unwrap().parse::<u8>().unwrap();
        // let mime = matches.value_of("MIME");
        // let sign = matches.is_present("sign");

        // split(secret_input, output_path, k, n, mime, sign, verbose)?
        split(secret_input, output_path, k, n, verbose)?
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
    mut secret_input: Input,
    output_path: &Path,
    k: u8,
    n: u8,
    // mime: Option<&str>,
    // sign: bool,
    verbose: bool,
) -> Result<()> {
    if k > n {
        bail!("k must be smaller than or equal to n")
    }

    verbose!(verbose, "Reading secret...");

    let mut secret = Vec::new();
    let size = secret_input
        .read_to_end(&mut secret)
        .chain_err(|| "Could not read secret")?;

    verbose!(verbose, "  Read {} bytes.", size);

    verbose!(verbose, "Generating shares... ");

    let shares = generate_shares(k, n, &secret)
        .chain_err(|| "Could not generate shares")?;

    for (i, share) in shares.iter().enumerate() {
        let mut path_buf = output_path.to_path_buf();
        path_buf.push(format!("share_{}", i));
        let share_path = path_buf.as_path();

        verbose!(verbose, "Writing share #{} to {:?}...", i, share_path);

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

        let mut share_file = File::open(share_path)
            .chain_err(|| format!("Could not open share {:?}", share_path))?;

        let mut share = String::new();
        let size = share_file
            .read_to_string(&mut share)
            .chain_err(|| format!("Could not read share {:?}", share_path))?;

        verbose!(verbose, "  Read {} bytes.", size);

        shares.push(share);
    }

    verbose!(verbose, "Recovering secret... ");

    let secret = recover_secret(shares)
        .chain_err(|| "Could not recover secret")?;

    match output_path {
        Some(output_path) => {
            let mut output_file = File::create(output_path)
                .chain_err(|| format!("Could not create secret file {:?}", output_path))?;
            output_file
                .write_all(&secret)
                .chain_err(|| "Could not write secret to file")?;
        }
        None => {
            let secret_str = String::from_utf8(secret)
                .chain_err(
                    || "Could not parse secret as UTF-8, consider outputting it to a file instead",
                )?;
            println!("{}", secret_str);
        }
    }

    Ok(())
}
