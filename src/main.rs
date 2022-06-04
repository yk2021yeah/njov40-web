use sycamore::prelude::*;

fn main() {
    sycamore::render(|| view! {
        p { "Hello, World!" }
        p {"aaaaaaaaaaaa"}
        a {"https://www.yahoo.com"}
    });
}
