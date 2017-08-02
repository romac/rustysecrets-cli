extern crate clap;
use clap::Shell;

use std::env;

include!("src/cli.rs");

fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };
    let mut app = build_cli();
    app.gen_completions("rustysecrets", Shell::Bash, &outdir);
    app.gen_completions("rustysecrets", Shell::Zsh, &outdir);
    app.gen_completions("rustysecrets", Shell::Fish, &outdir);
    app.gen_completions("rustysecrets", Shell::PowerShell, &outdir);
}

