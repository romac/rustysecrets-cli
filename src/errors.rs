
#![cfg_attr(feature = "cargo-clippy", allow(unused_doc_comment))]

use input::Input;

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! {

    errors {
        KMustBeSmallerThanN(k: u8, n: u8) {
            description("k must be smaller than or equal to n")
            display("k must be smaller than or equal to n, got: k = {}, n = {}", k, n)
        }
        CannotReadSecret(input: Input) {
            description("Cannot read secret")
            display("Cannot read secret from '{}'", input)
        }
        CannotOpenSecretFile(path: String) {
            description("cannot open secret file")
            display("cannot open secret file '{}'", path)
        }
        CannotCreateShareFile(path: String) {
            description("Cannot create share file")
            display("Cannot create share file '{}'", path)
        }
        CannotWriteShareDataToFile(path: String) {
            description("Cannot write share data to file")
            display("Cannot write share data to '{}'", path)
        }
        ShareDoesNotExists(path: String) {
            description("Share does not exists")
            display("Share does not exists: '{}'", path)
        }
        ShareIsNotAFile(path: String) {
            description("Given share is not a file")
            display("Share is not a file: '{}'", path)
        }
        CannotOpenShare(path: String) {
            description("Cannot open share")
            display("Cannot open share at '{}'", path)
        }
        CannotReadShare(path: String) {
            description("Cannot read share")
            display("Cannot read share at '{}'", path)
        }
        CannotRecoverSecret {
            description("Cannot recover secret")
            display("Cannot recover secret")
        }
        CannotCreateSecretFile(path: String) {
            description("Cannot write share data to file")
            display("Cannot create secret file '{}'", path)
        }
        CannotWriteSecretToFile(path: String) {
            description("Cannot write secret to file")
            display("Cannot write secret to '{}'", path)
        }
    }

}
