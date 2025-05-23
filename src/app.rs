use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (selected_filter, set_selected_filter) = signal("Images Only".to_string());
    let (search_query, set_search_query) = signal("magic at your fingertips".to_string()); 

    view! {
        <div class="app">
            <div class="main-container">
                <div class="header">
                    <div class="search-section">
                        <button class="back-button">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M19 12H5M12 19l-7-7 7-7"/>
                            </svg>
                        </button>
                        <span class="search-text">{search_query}</span>
                    </div>
                    <div class="filter-dropdown">
                        <select class="filter-select">
                            <option>"Images Only"</option>
                            <option>"All Files"</option>
                            <option>"Applications"</option>
                        </select>
                        <svg class="dropdown-icon" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M6 9l6 6 6-6"/>
                        </svg>
                    </div>
                </div>

                <div class="content-container">
                    // Left Sidebar
                    <div class="sidebar">
                        <div class="sidebar-section">
                            <h3 class="sidebar-title">"Today"</h3>
                            <div class="sidebar-item active">
                                <div class="item-icon">
                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
                                        <circle cx="9" cy="9" r="2"/>
                                        <path d="M21 15l-3.086-3.086a2 2 0 0 0-2.828 0L6 21"/>
                                    </svg>
                                </div>
                                <span class="item-text">"Image (842 × 420)"</span>
                            </div>
                        </div>
                    </div>

                    // Main Content
                    <div class="main-content">
                        <div class="preview-section">
                            <h1 class="preview-title">"Magic at"</h1>
                            <h1 class="preview-subtitle">"your fingertips"</h1>
                            
                            <div class="metadata">
                                <div class="metadata-row">
                                    <span class="metadata-label">"Information"</span>
                                </div>
                                <div class="metadata-row">
                                    <span class="metadata-label">"Application"</span>
                                </div>
                                <div class="metadata-row">
                                    <span class="metadata-label">"Content Type"</span>
                                </div>
                                <div class="metadata-row">
                                    <span class="metadata-label">"Dimensions"</span>
                                </div>
                            </div>

                            <div class="status-indicator">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M9 12l2 2 4-4"/>
                                    <path d="M21 12c-1 0-3-1-3-3s2-3 3-3 3 1 3 3-2 3-3 3"/>
                                    <path d="M3 12c1 0 3-1 3-3s-2-3-3-3-3 1-3 3 2 3 3 3"/>
                                </svg>
                                <span>"Copied Image to clipboard"</span>
                            </div>
                        </div>

                        // Actions Panel
                        <div class="actions-panel">
                            <div class="action-item">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                                    <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
                                </svg>
                                <span>"Copy to Clipboard"</span>
                                <div class="shortcut">
                                    <kbd>"⌘"</kbd>
                                </div>
                            </div>

                            <div class="action-item">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8"/>
                                    <polyline points="16,6 12,2 8,6"/>
                                    <line x1="12" y1="2" x2="12" y2="15"/>
                                </svg>
                                <span>"Share..."</span>
                                <div class="shortcut">
                                    <kbd>"⇧"</kbd><kbd>"⌘"</kbd><kbd>"E"</kbd>
                                </div>
                            </div>

                            <div class="action-item">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                                    <polyline points="14,2 14,8 20,8"/>
                                    <line x1="16" y1="13" x2="8" y2="13"/>
                                    <line x1="16" y1="17" x2="8" y2="17"/>
                                    <polyline points="10,9 9,9 8,9"/>
                                </svg>
                                <span>"Open With..."</span>
                                <div class="shortcut">
                                    <kbd>"⌘"</kbd><kbd>"O"</kbd>
                                </div>
                            </div>

                            <div class="action-item">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
                                    <circle cx="12" cy="12" r="3"/>
                                </svg>
                                <span>"Quick Look"</span>
                                <div class="shortcut">
                                    <kbd>"⌘"</kbd><kbd>"Y"</kbd>
                                </div>
                            </div>

                            <div class="action-item">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M12 17V3"/>
                                    <path d="M18 11l-6-6-6 6"/>
                                    <path d="M19 21H5"/>
                                </svg>
                                <span>"Pin Entry"</span>
                                <div class="shortcut">
                                    <kbd>"⇧"</kbd><kbd>"⌘"</kbd><kbd>"P"</kbd>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                // Bottom Section
                <div class="bottom-section">
                    <div class="bottom-left">
                        <div class="clipboard-indicator">
                            <div class="clipboard-icon">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                                    <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
                                </svg>
                            </div>
                            <span>"Clipboard History"</span>
                        </div>
                    </div>
                    
                    <div class="bottom-center">
                        <div class="app-icons">
                            <div class="app-icon">
                                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
                                    <rect x="9" y="9" width="6" height="6"/>
                                </svg>
                            </div>
                            <div class="app-icon">
                                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <polygon points="12,2 15.09,8.26 22,9.27 17,14.14 18.18,21.02 12,17.77 5.82,21.02 7,14.14 2,9.27 8.91,8.26"/>
                                </svg>
                            </div>
                            <div class="app-icon">
                                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <circle cx="12" cy="12" r="10"/>
                                    <circle cx="12" cy="12" r="6"/>
                                    <circle cx="12" cy="12" r="2"/>
                                </svg>
                            </div>
                            <div class="app-icon">
                                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
                                    <path d="M9 9h6v6H9z"/>
                                </svg>
                            </div>
                            <div class="app-icon">
                                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <rect x="3" y="3" width="7" height="7"/>
                                    <rect x="14" y="3" width="7" height="7"/>
                                    <rect x="14" y="14" width="7" height="7"/>
                                    <rect x="3" y="14" width="7" height="7"/>
                                </svg>
                            </div>
                        </div>
                    </div>

                    <div class="bottom-right">
                        <div class="bottom-actions">
                            <span>"Copy to Clipboard"</span>
                            <div class="shortcut">
                                <kbd>"⌘"</kbd>
                            </div>
                            <span class="separator">"|"</span>
                            <span>"Actions"</span>
                            <div class="shortcut">
                                <kbd>"⌘"</kbd><kbd>"K"</kbd>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
