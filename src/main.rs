use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack(support_nested)]
use b_query::query2;

#[proc_macro_hack(support_nested)]
use b_query::query;

fn main() {
    let _ = query2!("I don't work");
}
