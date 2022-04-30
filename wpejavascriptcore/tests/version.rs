#[test]
fn print_version() {
    println!(
        "WPEJavaScriptCore version {}.{}.{}",
        wpejavascriptcore::major_version(),
        wpejavascriptcore::minor_version(),
        wpejavascriptcore::micro_version(),
    );
}
