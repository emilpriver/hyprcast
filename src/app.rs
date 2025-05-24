use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use crate::search_engine::{Action, SearchEngine}; // Import new types

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (search_query, set_search_query) = signal("".to_string());
    let search_engine = store_value(SearchEngine::new());
    let (search_results, set_search_results) = signal(Vec::<Action>::new());

    create_effect(move |_| {
        let query = search_query.get();
        if query.trim().is_empty() {
            set_search_results.set(Vec::new());
        } else {
            let results = search_engine.with_value(|se| se.search(&query));
            set_search_results.set(results);
        }
    });

    let height_style = Memo::new(move |_| {
        if !search_query.get().is_empty() { // Keep content area visible if actively searching
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
                    <ul class="search-results">
                        {move || search_results.get().into_iter()
                            .map(|action| {
                                view! {
                                    <li class="search-result-item">
                                        <span class="action-name">{action.name}</span>
                                        <span class="action-description">{action.description}</span>
                                    </li>
                                }
                            })
                            .collect_view()
                        }
                    </ul>
                </div>

                <div class="bottom-section" style=move || height_style.get()>
                    // You could add a count of results or other info here
                    {move || if !search_results.get().is_empty() {
                        format!("{} results found", search_results.get().len())
                    } else if !search_query.get().is_empty() {
                        "No results found".to_string()
                    } else {
                        "".to_string()
                    }}
                </div>
            </div>
        </div>
    }
}
