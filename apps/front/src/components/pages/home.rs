use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
  view! {cx,
    <div>Home here:
      <button class="p-3 bg-red-800 rounded-md text-white" hx-get="/"
        hx-trigger="click"
        hx-target="#root"
        hx-swap="innerHTML"
      >
        Click Me! test
      </button>
    </div>
  }
}
