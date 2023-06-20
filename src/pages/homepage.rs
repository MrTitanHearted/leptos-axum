use leptos::*;
use crate::components::Navbar;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <div class="static hero min-h-screen bg-base-200">
            <Navbar />
            <div class="hero-content flex-col lg:flex-row">
                <img src="/images/ironman.png" class="max-w-sm rounded-lg shadow-2xl" />
                <div>
                    <h1 class="text-5xl font-bold">"Welcome to Leptos!"</h1>
                    <p class="py-6">"Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi."</p>
                    <button class="btn btn-primary" on:click=on_click>"Click Me: " {count}</button>
                    <a class="pl-3 link link-accent" href="/login">"Log in"</a>
                </div>
            </div>
        </div>
    }
}