use leptos_mview::mview;

fn main() {
    _ = mview! {
        div {
            "something"
            input.input type="text"
        }
    };
    compile_error!("test warnings");
}
