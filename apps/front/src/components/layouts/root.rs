use leptos::*;

#[component]
pub fn RootLayout(
  #[prop(default = "Todos".to_string())] title: String,
  live_reload: bool,
  styles_id: String,
  children: Children,
) -> impl IntoView {
  let live_reload_script = if live_reload {
    Some(view! {<script src="/static/live_reload.js"></script>})
  } else {
    None
  };

  view! {
    <html lang="en">
      <head>
        <title>{title}</title>
        <link rel="stylesheet" href="/static/styles.css" />
      </head>
      <body>
        <noscript>"You need to enable JavaScript to run this app."</noscript>
        <div id="root">
          {children()}
        </div>
        <script src="https://unpkg.com/htmx.org@1.9.3"></script>
        {live_reload_script}
      </body>
    </html>
  }
}
