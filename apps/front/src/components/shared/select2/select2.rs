use leptos::*;

#[component]
pub fn Select2() -> impl IntoView {
  view! {
    <div class="mt-4 flex-1" x-data="select2" x-init="setInitial(0)">
      <x-one @click="">here</x-one>
    </div>
  }
}
