use leptos::*;
use leptos_mview::mview;

fn style_on_component() {
    mview! {
        Component style:color="white";
    };
}

fn class_on_component() {
    mview! {
        Component class:red={true};
    };
}

fn prop_on_component() {
    mview! {
        Component prop:value="1";
    };
}

#[component]
fn SpreadOnComponent() -> impl IntoView {
    #[allow(unused_variables)]
    let attrs = vec![
        ("class", "something"),
        ("data", "a"),
    ];
    mview! {
        Component {..attrs};
    };
}

fn attr_on_element() {
    mview! {
        input attr:class="no" type="text";
    };
}

fn clone_on_element() {
    let notcopy = String::new();
    mview! {
        div {
            span clone:notcopy {
                {notcopy.clone()}
            }
        }
    };
}

fn sel_shorthand_on_components() {
    mview! {
        Component.not-working #some-id;
    };
}

#[component]
fn Component() -> impl IntoView {
    mview! {
        button;
    };
}

fn main() {}
