#![deny(missing_docs)]
//! A simple key/value store.

pub use crate::kv::kvs::KvStore;

mod kv;

use clap::{Arg, Command};
use std::error::Error;
use std::process;

type KvsResult<T> = Result<T, Box<dyn Error>>;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

const SET_ID: &str = "set";
const GET_ID: &str = "get";
const RM_ID: &str = "rm";

/// Entry point to run this application
pub fn run() -> KvsResult<()> {
    let set_args = Arg::new(SET_ID)
        .help("Set the value of a string key to a string")
        .num_args(2)
        .value_names(vec!["key", "value"])
        .required(true);

    let get_arg = Arg::new(GET_ID)
        .help("key used to get the value")
        .value_name("key")
        .num_args(1)
        .required(true);

    let remove_arg = Arg::new(RM_ID)
        .help("key used to remove value from store")
        .value_name("key")
        .num_args(1)
        .required(true);

    let set_cmd: Command = Command::new(SET_ID)
        .about("Adds a key and value to the cache")
        .args(vec![set_args]);

    let get_cmd: Command = Command::new(GET_ID)
        .about("gets the string value of the given key")
        .args(vec![get_arg]);

    let remove_cmd: Command = Command::new(RM_ID)
        .about("removes a given key from the cache")
        .args(vec![remove_arg]);

    let matches = Command::new(PKG_NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .subcommands(vec![set_cmd, get_cmd, remove_cmd])
        .get_matches();

    match matches.subcommand() {
        Some((SET_ID, sub_matches)) => {
            let _pair = sub_matches
                .get_many(SET_ID)
                .expect(
                    "Array of strings with key as first element and the value as second element",
                )
                .cloned()
                .collect::<Vec<String>>();

            eprintln!("unimplemented");
            process::exit(1);
        }
        Some((GET_ID, sub_matches)) => {
            let _key = sub_matches.get_one::<String>(GET_ID).cloned();

            eprintln!("unimplemented");
            process::exit(1);
        }
        Some((RM_ID, sub_matches)) => {
            let _key = sub_matches.get_one::<String>(RM_ID).cloned();

            eprintln!("unimplemented");
            process::exit(1);
        }
        _ => unreachable!(),
    }
}
