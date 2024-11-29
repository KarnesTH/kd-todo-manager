use leptos::*;

#[component]
pub fn TodoCard(
    id: u64,
    title: String,
    description: String,
    is_add_card: bool,
    on_todo_click: Action<u64, ()>,
) -> impl IntoView {
    let class_names = if is_add_card {
        "p-4 rounded-lg border-2 border-dashed border-primary-200 bg-background-card hover:shadow-lg transition-all duration-200"
    } else {
        "flex flex-col justify-center p-4 rounded-lg border-2 border-solid border-primary-300 bg-background-card hover:shadow-lg transition-all duration-200"
    };

    view! {
        <div class={class_names}>
            {if is_add_card {
                view! {
                    <div class="flex flex-col items-center justify-center h-full gap-2">
                        <img src="public/icons/plus.svg" alt="Add" class="w-8 h-8" />
                        <span class="text-gray-500">"Add Todo"</span>
                    </div>
                }.into_view()
            } else {
                view! {
                    <h2 class="text-2xl bg-gradient-primary text-transparent bg-clip-text text-center font-bold mb-2">{title.clone()}</h2>
                    <p class="text-white mb-4">{description.clone()}</p>
                    <button
                        class="bg-gradient-primary text-white px-4 py-2 rounded-full hover:opacity-90 transition-opacity"
                        on:click=move |_| {
                            on_todo_click.dispatch(id);
                        }
                    >
                        "Go to Todo"
                    </button>
                }.into_view()
            }}
        </div>
    }
}
