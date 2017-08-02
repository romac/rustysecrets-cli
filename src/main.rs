
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
use rusty_secrets::sss;

#[macro_use]
mod verbose;

mod errors;
use errors::*;

mod input;
use input::Input;

mod cli;

use std::path::Path;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() {
    if let Err(ref e) = run() {
        eprintln!("{} {}", "    error:".red().bold(), e);

        for e in e.iter().skip(1) {
            eprintln!("{} {}", "caused by:".yellow().bold(), e);
        }

        if let Some(backtrace) = e.backtrace() {
            eprintln!("{} {:?}", "backtrace:".blue().bold(), backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let matches = cli::build_cli().get_matches();

    if let Some(matches) = matches.subcommand_matches("split") {
        let secret_arg = matches.value_of("INPUT").unwrap();
        let secret_input = if secret_arg == "-" {
            Input::stdin()
        } else {
            Input::file(secret_arg.to_string())
                .chain_err(|| ErrorKind::CannotOpenSecretFile(secret_arg.to_string()))?
        };

        let output_path = Path::new(matches.value_of("DIR").unwrap());
        let k = matches.value_of("k").unwrap().parse::<u8>().unwrap();
        let n = matches.value_of("n").unwrap().parse::<u8>().unwrap();
        let share_tmpl = matches.value_of("share-tmpl").unwrap_or("share_{{num}}");
        let verbose = matches.is_present("verbose");
        // let mime = matches.value_of("MIME");
        let sign_shares = matches.is_present("sign");

        split(secret_input, output_path, k, n, sign_shares, share_tmpl, verbose)?
    }
    else if let Some(matches) = matches.subcommand_matches("recover") {
        let shares = matches
            .values_of("SHARES")
            .unwrap()
            .map(Path::new)
            .collect();

        let output_path = matches.value_of("FILE").map(Path::new);
        let verify_signatures = matches.is_present("verify");
        let verbose = matches.is_present("verbose");

        recover(shares, output_path, verify_signatures, verbose)?
    }

    Ok(())
}

fn split(
    mut secret_input: Input,
    output_path: &Path,
    k: u8,
    n: u8,
    // mime: Option<&str>,
    sign_shares: bool,
    share_tmpl: &str,
    verbose: bool,
) -> Result<()> {
    if k > n {
        bail!(ErrorKind::KMustBeSmallerThanN(k, n))
    }

    verbose!(verbose, "Reading secret...");

    let mut secret = Vec::new();
    let size = secret_input
        .read_to_end(&mut secret)
        .chain_err(|| ErrorKind::CannotReadSecret(secret_input))?;

    verbose!(verbose, "  Read {} bytes.", size);

    verbose!(verbose, "Generating shares... ");

    let shares = sss::generate_shares(k, n, &secret, sign_shares)
        .chain_err(|| "Could not generate shares")?;

    for (num, share) in shares.iter().enumerate() {
        let mut path_buf = output_path.to_path_buf();
        path_buf.push(share_tmpl.replace("{{num}}", &format!("{}", num)));
        let share_path = path_buf.as_path();

        verbose!(verbose, "Writing share #{} to {:?}...", num, share_path);

        let mut share_file = File::create(share_path)
            .chain_err(|| ErrorKind::CannotCreateShareFile(format!("{}", share_path.display())))?;

        share_file
            .write_all(share.as_bytes())
            .chain_err(|| ErrorKind::CannotWriteShareDataToFile(format!("{}", share_path.display())))?;
    }

    Ok(())
}

fn recover(shares_paths: Vec<&Path>, output_path: Option<&Path>, verify_signatures: bool, verbose: bool) -> Result<()> {
    let mut shares = Vec::with_capacity(shares_paths.len());

    for share_path in shares_paths {
        if !share_path.exists() {
            bail!(ErrorKind::ShareDoesNotExists(format!("{}", share_path.display())))
        }
        if !share_path.is_file() {
            bail!(ErrorKind::ShareIsNotAFile(format!("{}", share_path.display())))
        }

        verbose!(verbose, "Reading share {:?}... ", share_path);

        let mut share_file = File::open(share_path)
            .chain_err(|| ErrorKind::CannotOpenShare(format!("{}", share_path.display())))?;

        let mut share = String::new();
        let size = share_file
            .read_to_string(&mut share)
            .chain_err(|| ErrorKind::CannotReadShare(format!("{}", share_path.display())))?;

        verbose!(verbose, "  Read {} bytes.", size);

        shares.push(share);
    }

    verbose!(verbose, "Recovering secret... ");

    let secret = sss::recover_secret(shares, verify_signatures)
        .chain_err(|| ErrorKind::CannotRecoverSecret)?;

    match output_path {
        Some(output_path) => {
            let mut output_file = File::create(output_path)
                .chain_err(|| ErrorKind::CannotCreateSecretFile(format!("{}", output_path.display())))?;
            output_file
                .write_all(&secret)
                .chain_err(|| ErrorKind::CannotWriteSecretToFile(format!("{}", output_path.display())))?;
        }
        None => {
            // See https://github.com/romac/rustysecrets-cli/issues/9
            // let secret_str = String::from_utf8(secret)
            //     .chain_err(|| "Could not parse secret as UTF-8, consider outputting it to a file instead")?;

            io::stdout()
                .write_all(&secret)
                .chain_err(|| "Could not write output to stdout")?;
        }
    }

    Ok(())
}
