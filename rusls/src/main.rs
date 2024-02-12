fn main() {
    if let Err(e) = rusls::get_args().and_then(rusls::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
