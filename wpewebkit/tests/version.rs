#[test]
fn print_version() {
    println!(
        "WPEWebKit version {}.{}.{}",
        wpewebkit::major_version(),
        wpewebkit::minor_version(),
        wpewebkit::micro_version(),
    );
}
