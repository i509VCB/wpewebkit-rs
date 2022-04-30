use wpe::ViewBackend;
use wpe_java_script_core::traits::ValueExt;
use wpewebkit::{WebView, WebViewBackend, WebViewBackendManual, WebViewExt, Settings};

fn main() {
    let view_backend = WebViewBackend::new(NoopBackend);

    let settings = Settings::builder()
        .enable_javascript(true)
        .build();

    let webview = WebView::builder()
        .is_ephemeral(true)
        .settings(&settings)
        .backend(&view_backend)
        .build();

    println!("load uri");
    webview.load_uri("https://crates.io");

    println!("zoom: {}", webview.zoom_level());

    webview.set_is_muted(true);
    println!("muted: {}", webview.is_muted());

    if let Some(v) = webview.title() {
        println!("{}", v.as_str());
    }

    // FIXME: Why is running JS causing a segfault?
    println!("run js");
    webview.run_javascript("42", None::<&gio::Cancellable>, move |result| match result {
        Ok(result) => {
            if let Some(value) = result.js_value() {
                println!("is_boolean: {}", value.is_boolean());
                println!("is_number: {}", value.is_number());
                println!("{:?}", value.to_int32());
                println!("{:?}", value.to_boolean());
            } else {
                eprintln!("None value?");
            }

            println!("Barrier wait");
        }

        Err(error) => {
            println!("{}", error);
            println!("Barrier wait");
        },
    });

    std::thread::sleep_ms(1000);
    println!("Foo");
}

struct NoopBackend;

impl ViewBackend for NoopBackend {}
