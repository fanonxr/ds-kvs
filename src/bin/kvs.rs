fn main() {
    if let Err(e) = kvs::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
