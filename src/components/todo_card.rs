use leptos::*;

#[component]
pub fn TodoCard(title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <div class="todo-card">
            <h2>{title}</h2>
            <p>{description}</p>
            <button>
                "Go to Todo"
            </button>
        </div>
    }
}
