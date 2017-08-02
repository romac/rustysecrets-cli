
# RustySecrets CLI

[![Build Status](https://travis-ci.org/SpinResearch/rustysecrets-cli.svg?branch=master&style=flat)](https://travis-ci.org/SpinResearch/rustysecrets-cli)
[![Issues](http://img.shields.io/github/issues/SpinResearch/rustysecrets-cli.svg?style=flat)](https://github.com/SpinResearch/rustysecrets-cli/issues)
![License](https://img.shields.io/badge/license-bsd3-brightgreen.svg?style=flat)
[![Crates.io](https://img.shields.io/crates/v/rustysecrets-cli.svg)](https://crates.io/crates/rustysecrets-cli)

> *rustysecrets-cli* is a command-line wrapper around [RustySecrets](https:github.com/SpinResearch/RustySecrets), a Rust implementation of threshold [Shamir's secret sharing](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing).

## Installation

    $ cargo install rustysecrets-cli

## Usage

```
$ mkdir shares
$ cat > secret.txt
These programs were never about terrorism: they’re about economic spying,
social control, and diplomatic manipulation. They’re about power.
^D

$ rustysecrets split secret.txt -o shares -k 7 -n 10
$ ls shares/
share_0 share_1 share_2 share_3 share_4 share_5 share_6 share_7 share_8 share_9
$ rustysecrets recover shares/share_*
These programs were never about terrorism: they’re about economic spying,
social control, and diplomatic manipulation. They’re about power.
```

## Documentation

 ### `$ rustysecrets help`

```
RustSecrets CLI 0.1
SpinResearch
Split a secret of an arbitrary length in n different shares and k-out-of-n shares are required to recover it.

USAGE:
    rustysecrets [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Enable verbose mode

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    recover    Recover the secret from the shares
    split      Split a secret into shares
```

 ### `$ rustysecrets help split`

```
Split a secret into shares

USAGE:
    rustysecrets split <INPUT> --output <DIR> -k <k> -n <n>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <DIR>    The directory to output the shares to
    -k <k>                Number of shares necessary to recover the secret
    -n <n>                Total number of generated shares

ARGS:
    <INPUT>    The file containing the secret to split
```

### `$ rustysecrets help recover`

```
Recover the secret from the shares

USAGE:
    rustysecrets recover [OPTIONS] <SHARES>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <FILE>    The file to output the secret to, printed on stdout otherwise

ARGS:
    <SHARES>...    The shares to recover the secret from
```

## Bug Reporting

Please report bugs either as pull requests or as issues in [the issue
tracker](https://github.com/SpinResearch/rustysecrets-cli). *rustysecrets-cli* has a
**full disclosure** vulnerability policy. **Please do NOT attempt to report
any security vulnerability in this code privately to anybody.**

## License

RustySecrets CLI is released under the BSD3 license. See [LICENSE](LICENSE) for more informations.

