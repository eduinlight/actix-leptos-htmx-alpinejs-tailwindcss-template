use serde::Serialize;
use tera::Result;

use crate::utils::http::responses::render_tera_template;

#[derive(Serialize)]
pub struct HomeProps {}

pub fn home(props: HomeProps) -> Result<String> {
  render_tera_template(include_str!("./home.html"), &props)
}
