use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
  view! {
    <div>Home here:
      <button class="p-2 bg-red-500 rounded-md text-white" hx-get="/"
        hx-trigger="click"
        hx-target="#root"
        hx-swap="innerHTML"
      >
        Nice
      </button>
    </div>
  }
}
