use leptos::*;

#[component]
pub fn RootLayout(
  cx: Scope,
  #[prop(optional)] title: Option<String>,
  children: Children,
) -> impl IntoView {
  view! { cx,
    <html lang="en">
      <head>
        <title>{title.unwrap_or("Todos".to_string())}</title>
        <script src="https://unpkg.com/htmx.org@1.9.3"></script>
        <link rel="stylesheet" href="/static/styles.css" />
      </head>
      <body>
        <noscript>"You need to enable JavaScript to run this app."</noscript>
        <div id="root">
          {children(cx)}
        </div>
      </body>
    </html>
  }
}
