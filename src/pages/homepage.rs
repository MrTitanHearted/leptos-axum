use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1 class="font-bold text-lg text-white-600">"Welcome to Leptos!"</h1>
        <button class="btn btn-primary text-red-500 text-6xl" on:click=on_click>"Click Me: " {count}</button>
    }
}