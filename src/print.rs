pub fn error_and_exit(e: &dyn std::error::Error) -> ! {
    eprintln!("{}", e);
    std::process::exit(1);
}
