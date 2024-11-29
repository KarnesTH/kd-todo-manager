use crate::Todo;
use leptos::*;

#[component]
pub fn DetailTodoModal(
    show: ReadSignal<bool>,
    set_show: WriteSignal<bool>,
    todo: ReadSignal<Option<Todo>>,
) -> impl IntoView {
    view! {
        {move || show.get().then(|| view! {
            <div class="fixed inset-0 z-50" on:click=move |_| set_show.set(false)>
                <div class="absolute inset-0 bg-gray-800/25 backdrop-blur-sm"></div>

                <div class="relative min-h-screen flex items-center justify-center p-4">
                    {move || todo.get().map(|t| view! {
                        <div
                            class="bg-background-card rounded-xl shadow-xl w-full max-w-2xl border border-primary-200"
                            on:click=move |e| e.stop_propagation()
                        >
                            <div class="flex justify-between items-center p-6 border-b border-primary-300">
                                <div>
                                    <h2 class="text-2xl font-semibold text-white mb-1">
                                        {t.title}
                                    </h2>
                                    <p class="text-sm text-primary-200">
                                        {"Created: "} {t.created_at}
                                    </p>
                                </div>
                                <button
                                    class="text-primary-200 hover:text-primary-100 transition-colors"
                                    on:click=move |_| set_show.set(false)
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                                    </svg>
                                </button>
                            </div>

                            <div class="p-6">
                                <div class="mb-6">
                                    <h3 class="text-lg font-medium text-white mb-2">Description</h3>
                                    <p class="text-primary-100">
                                        {t.description.unwrap_or_else(|| "No description provided.".to_string())}
                                    </p>
                                </div>
                            </div>
                        </div>
                    })}
                </div>
            </div>
        })}
    }
}
