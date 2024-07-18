use serde::Serialize;
use tera::Result;

use crate::utils::http::responses::render_tera_template;

#[derive(Serialize, Clone)]
pub struct RootLayoutProps {
  pub live_reload: bool,
  pub server_version: String,
  pub title: String,
  pub children: Option<String>,
  pub live_reload_port: i16,
}

pub fn root_layout(props: RootLayoutProps) -> Result<String> {
  render_tera_template(include_str!("./root.html"), &props)
}
