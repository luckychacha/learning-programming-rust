use crate::types::selector::Selector;

mod types;

fn main() {
    let s = Selector {
        elements: vec!["one", "two", "three"],
        current: 0,
    };
    util_traits::show_it(&s);
}