use leptos::*;

#[component]
pub fn AddTodoModal(
    show: ReadSignal<bool>,
    set_show: WriteSignal<bool>,
    on_save: Action<(String, Option<String>), ()>,
) -> impl IntoView {
    let (title, set_title) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());

    view! {
        {move || show.get().then(|| view! {
            <div class="fixed inset-0 z-50" on:click=move |_| set_show.set(false)>
                <div class="absolute inset-0 bg-gray-800/25 backdrop-blur-sm"></div>

                <div class="relative min-h-screen flex items-center justify-center p-4">
                    <div
                        class="bg-background-card rounded-xl shadow-xl w-full max-w-md border border-primary-200"
                        on:click=move |e| e.stop_propagation()
                    >
                        <div class="flex justify-between items-center p-6 border-b border-primary-300">
                            <h2 class="text-2xl font-semibold text-white">
                                "Create New Todo"
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

                        <div class="p-6 space-y-4">
                            <div class="space-y-2">
                                <label for="title" class="block text-sm font-medium text-white">
                                    "Title"
                                </label>
                                <input
                                    id="title"
                                    type="text"
                                    class="w-full px-3 py-2 bg-background border border-primary-300 rounded-md
                                           text-white placeholder-gray-400
                                           focus:outline-none focus:ring-2 focus:ring-primary-200 focus:border-primary-200"
                                    placeholder="Enter todo title"
                                    on:input=move |ev| set_title.set(event_target_value(&ev))
                                    prop:value=move || title.get()
                                />
                            </div>

                            <div class="space-y-2">
                                <label for="description" class="block text-sm font-medium text-white">
                                    "Description"
                                </label>
                                <textarea
                                    id="description"
                                    class="w-full px-3 py-2 bg-background border border-primary-300 rounded-md
                                           text-white placeholder-gray-400
                                           focus:outline-none focus:ring-2 focus:ring-primary-200 focus:border-primary-200"
                                    rows="4"
                                    placeholder="Enter todo description"
                                    on:input=move |ev| set_description.set(event_target_value(&ev))
                                    prop:value=move || description.get()
                                />
                            </div>
                        </div>

                        <div class="flex justify-end gap-3 p-6 border-t border-primary-300 rounded-b-xl">
                            <button
                                class="px-4 py-2 text-sm font-medium text-white border border-primary-200
                                       rounded-md hover:border-primary-100 transition-all"
                                on:click=move |_| set_show.set(false)
                            >
                                "Cancel"
                            </button>
                            <button
                                class="px-4 py-2 text-sm font-medium text-white bg-gradient-primary
                                       rounded-md hover:opacity-90 transition-opacity"
                            on:click=move |_| {
                                on_save.dispatch((title.get(), Some(description.get())));
                                set_title.set(String::new());
                                set_description.set(String::new());
                                set_show.set(false);
                            }
                            >
                                "Create Todo"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        })}
    }
}
