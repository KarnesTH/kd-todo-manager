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
                            <div class="flex flex-col justify-between p-6 border-b border-primary-300">
                                <div class="flex justify-between">
                                    <h2 class="text-2xl font-semibold text-white">
                                        {t.title}
                                    </h2>
                                    <button
                                        class="text-primary-200 hover:text-primary-100 transition-colors"
                                        on:click=move |_| set_show.set(false)
                                    >
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                                        </svg>
                                    </button>
                                </div>
                                <div class="flex justify-between mt-4">
                                    <p class="text-sm text-primary-100">
                                        {"Created: "}
                                        <span class="text-white text-sm">       {t.created_at}
                                        </span>
                                    </p>
                                    <p class="text-sm text-primary-100">
                                        {"Updated: "}
                                        <span class="text-white text-sm">       {t.updated_at}
                                        </span>
                                    </p>
                                </div>
                            </div>
                            <div class="p-6 space-y-6">
                                <div>
                                    <h3 class="text-lg font-medium text-primary-100 mb-2">
                                        "Description"
                                    </h3>
                                    <p class="text-white whitespace-pre-wrap">
                                        {t.description.unwrap_or_else(|| "No description provided.".to_string())}
                                    </p>
                                </div>
                                <div>
                                    <div class="flex justify-between items-center mb-3">
                                        <h3 class="text-lg font-medium text-primary-100">"Tasks"</h3>
                                        <span class="text-sm text-primary-200">{"0/0 Completed"}</span>
                                    </div>
                                    <div class="space-y-2 max-h-60 overflow-y-auto">
                                        <div class="flex items-center gap-3 p-3 bg-background border border-primary-300 rounded-md">
                                            <input
                                                type="checkbox"
                                                class="form-checkbox h-5 w-5 rounded border-primary-300
                                                       text-primary-500 focus:ring-primary-200
                                                       bg-background"
                                            />
                                            <span class="flex-1 text-white">"Example Task"</span>
                                            <button class="text-primary-200 hover:text-primary-100 transition-colors">
                                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
                                                </svg>
                                            </button>
                                            <button class="text-primary-200 hover:text-red-400 transition-colors">
                                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                                                </svg>
                                            </button>
                                        </div>
                                    </div>
                                    <div class="mt-4">
                                        <div class="w-full bg-background rounded-full h-2">
                                            <div class="bg-gradient-primary h-2 rounded-full" style="width: 0%"></div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div class="flex justify-end gap-3 p-6 border-t border-primary-300">
                                <button
                                    class="px-4 py-2 text-sm font-medium text-red-500 border border-red-500
                                           rounded-md hover:bg-red-500 hover:text-white transition-all"
                                >
                                    "Delete Todo"
                                </button>
                                <button
                                    class="px-4 py-2 text-sm font-medium text-white bg-gradient-primary
                                           rounded-md hover:opacity-90 transition-opacity"
                                >
                                    "Edit Todo"
                                </button>
                            </div>
                        </div>
                    })}
                </div>
            </div>
        })}
    }
}
