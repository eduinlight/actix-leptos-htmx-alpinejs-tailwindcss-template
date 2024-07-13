use leptos::*;

#[component]
pub fn RootLayout(
  #[prop(default = "Todos".to_string())] title: String,
  live_reload: bool,
  server_version: String,
  children: Children,
) -> impl IntoView {
  let live_reload_script = if live_reload {
    Some(view! {<script src=format!("/static/live_reload.js?{}", server_version)></script>})
  } else {
    None
  };

  view! {
    <html lang="en">
      <head>
        <title>{title}</title>
        <link rel="stylesheet" href=format!("/static/styles.css?{}", server_version) />
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
