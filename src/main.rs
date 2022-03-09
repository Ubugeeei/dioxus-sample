use dioxus::prelude::*;
fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let (count, set_count) = use_state(&cx, || 0);

    cx.render(rsx! {
        div { "hello" }
        p { "{count}" }
        button { onclick: move |_| set_count(count + 1), "+1" }
    })
}
