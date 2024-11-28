use leptos::*;

#[component]
pub fn TodoCard(title: String, description: String, is_add_card: bool) -> impl IntoView {
    let class_names = if is_add_card {
        "p-4 rounded-lg border-2 border-dashed border-primary-200 bg-background-card hover:shadow-lg transition-all duration-200"
    } else {
        "p-4 rounded-lg border-2 border-solid border-primary-300 bg-background-card hover:shadow-lg transition-all duration-200"
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
                    <h2 class="text-xl font-bold mb-2">{title}</h2>
                    <p class="text-gray-600 mb-4">{description}</p>
                    <button class="bg-gradient-primary text-white px-4 py-2 rounded-lg hover:opacity-90 transition-opacity">
                        "Go to Todo"
                    </button>
                }.into_view()
            }}
        </div>
    }
}
