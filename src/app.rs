use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Clone)]
struct Todos {
    title: &'static str,
    description: &'static str,
    todo_tasks: Vec<TodoTasks>,
}

#[derive(Debug, Clone)]
struct TodoTasks {
    title: &'static str,
    completed: bool,
    description: &'static str,
}

#[component]
pub fn App() -> impl IntoView {
    let _todos = vec![
        Todos {
            title: "Todo 1",
            description: "This is the first todo",
            todo_tasks: vec![
                TodoTasks {
                    title: "Task 1",
                    completed: false,
                    description: "This is the first task",
                },
                TodoTasks {
                    title: "Task 2",
                    completed: true,
                    description: "This is the second task",
                },
            ],
        },
        Todos {
            title: "Todo 2",
            description: "This is the second todo",
            todo_tasks: vec![
                TodoTasks {
                    title: "Task 1",
                    completed: false,
                    description: "This is the first task",
                },
                TodoTasks {
                    title: "Task 2",
                    completed: true,
                    description: "This is the second task",
                },
            ],
        },
    ];

    view! {
        <main class="container text-center mx-auto p-4">
            <h1 class="text-4xl font-bold">"Todo Manager"</h1>
            <div class="flex flex-row p-4">
                <button class="flex bg-gray-600 p-2 rounded-full gap-2">
                    <img src="public/icons/filter.svg" alt="Filter icon" class="w-6 h-6" />
                    <span class="text-white font-bold">"Filter"</span>
                </button>
            </div>
        </main>
    }
}
