use kd_todo_manager_ui::{
    components::{modals::DetailTodoModal, AddTodoModal},
    Todo, TodoCard,
};
use leptos::*;
use leptos_dom::logging::console_log;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

async fn get_todos() -> Result<Vec<Todo>, String> {
    let result = invoke("get_todos", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

async fn add_todo(title: String, description: Option<String>) -> Result<Todo, String> {
    let payload = to_value(&serde_json::json!({
        "newTodo" : {
            "title": title,
            "description": description,
        }
    }))
    .map_err(|e| e.to_string())?;

    let result = invoke("add_todo", payload).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

async fn get_todo(id: u64) -> Result<Todo, String> {
    let payload = to_value(&serde_json::json!({
        "todo" : {
            "id": id,
        }
    }))
    .map_err(|e| e.to_string())?;
    let result = invoke("get_todo", payload).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

#[component]
pub fn App() -> impl IntoView {
    let (todos, set_todos) = create_signal(Vec::new());
    let (show_add_modal, set_show_add_modal) = create_signal(false);
    let (show_detail_modal, set_show_detail_modal) = create_signal(false);
    let (selected_todo, set_selected_todo) = create_signal(None::<Todo>);

    create_effect(move |_| {
        spawn_local(async move {
            if let Ok(fetched_todos) = get_todos().await {
                set_todos.set(fetched_todos);
            }
        });
    });

    let handle_add_todo = create_action(move |input: &(String, Option<String>)| {
        let (title, description) = (input.0.clone(), input.1.clone());
        async move {
            if let Ok(new_todo) = add_todo(title, description).await {
                set_todos.update(|todos| {
                    todos.push(new_todo);
                });
            }
        }
    });

    let handle_todo_click = create_action(move |id: &u64| {
        let id = *id;
        async move {
            match get_todo(id).await {
                Ok(todo) => {
                    set_selected_todo.set(Some(todo));
                    set_show_detail_modal.set(true);
                }
                Err(_) => {
                    console_log("Error loading todo: {}");
                }
            }
        }
    });

    view! {
        <main class="min-h-screen bg-background p-4">
            <h1 class="text-4xl font-bold text-center mb-8 bg-gradient-primary text-transparent bg-clip-text">
                "Todo Manager"
            </h1>

            <div class="flex justify-between items-center mb-6">
                <button class="flex items-center gap-2 px-4 py-2 rounded-lg text-white
                              border border-transparent hover:border-primary-200
                              transition-all duration-200">
                    <img src="public/icons/filter.svg" alt="Filter" class="w-6 h-6" />
                    <span>"Filter"</span>
                </button>
                <button class="flex items-center gap-2 px-4 py-2 rounded-lg text-white
                              border border-transparent hover:border-primary-200
                              transition-all duration-200">
                    <img src="public/icons/gear.svg" alt="Settings" class="w-6 h-6" />
                    <span>"Settings"</span>
                </button>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 overflow-auto">
                {move || todos.get().into_iter().map(|todo| {
                    view! {
                        <TodoCard
                            id={todo.id}
                            title={todo.title}
                            description={todo.description.unwrap_or_default()}
                            is_add_card=false
                            on_todo_click=handle_todo_click
                        />
                    }
                }).collect::<Vec<_>>()}

                <div on:click=move |_| set_show_add_modal.set(true)>
                    <TodoCard
                        id={0}
                        title="".to_string()
                        description="".to_string()
                        is_add_card=true
                        on_todo_click=create_action(|_| async {})
                    />
                </div>
                <AddTodoModal
                    show=show_add_modal
                    set_show=set_show_add_modal
                    on_save=handle_add_todo
                />
                <DetailTodoModal
                    show=show_detail_modal
                    set_show=set_show_detail_modal
                    todo=selected_todo
                />
            </div>
        </main>
    }
}
