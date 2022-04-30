use wpewebkit::{major_version, minor_version, micro_version};

#[test]
fn print_version() {
    println!(
        "WPEWebKit version {}.{}.{}",
        major_version(),
        minor_version(),
        micro_version(),
    );
}
