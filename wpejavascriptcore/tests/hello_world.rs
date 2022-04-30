use wpejavascriptcore::{
    traits::{ContextExt, ValueExt},
    Context,
};

#[test]
fn forty_two() {
    let context = Context::new();

    match context.evaluate("42") {
        Some(value) => {
            println!("is_boolean: {}", value.is_boolean());
            println!("is_number: {}", value.is_number());
            println!("{:?}", value.to_int32());
            println!("{:?}", value.to_boolean());
        }
        None => println!("eval error"),
    };
}
