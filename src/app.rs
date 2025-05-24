use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (search_query, set_search_query) = signal("".to_string());

    let height_style = Memo::new(move |_| {
        if !search_query.get().is_empty() {
            "height: 500px;".to_string()
        } else {
            "height: 0;".to_string()
        }
    });

    view! {
        <div class="app">
            <div class="main-container">
                <div class="header">
                    <div class="search-section">
                        <input
                            placeholder="The answer to your action is at your fingertips"
                            class="search-text"
                            type="text"
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_search_query.set(value);
                            }
                            prop:value=search_query
                        />
                    </div>
                </div>

                <div class="content-container" style=move || height_style.get()>

                </div>

                <div class="bottom-section" style=move || height_style.get()>

                </div>
            </div>
        </div>
    }
}
