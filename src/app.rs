use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Clone)]
struct Todos {
    todo_item: Vec<TodoItem>,
}

#[derive(Debug, Clone)]
struct TodoItem {
    title: &'static str,
    completed: bool,
    description: &'static str,
}

#[component]
pub fn App() -> impl IntoView {
    let todos = Todos {
        todo_item: vec![
            TodoItem {
                title: "Learn Rust",
                completed: true,
                description: "Learn Rust Programming Language",
            },
            TodoItem {
                title: "Learn Leptos",
                completed: false,
                description: "Learn Leptos Web Framework",
            },
            TodoItem {
                title: "Learn WebAssembly",
                completed: false,
                description: "Learn WebAssembly Programming Language",
            },
        ],
    };

    view! {
        <main class="container">
            <h1>"Todo Manager"</h1>

            <div class="row">
                <button>
                    "Add Todo"
                </button>
                <button>
                    "Filter"
                </button>
            </div>

            <ul>
                {todos.todo_item.into_iter().map(|todo| view! {
                    <li>
                        <div class="row">
                            <input type="checkbox" checked={todo.completed} />
                            <h3>{todo.title}</h3>
                        </div>
                        <p>{todo.description}</p>
                    </li>
                }).collect_view()}
            </ul>
        </main>
    }
}
