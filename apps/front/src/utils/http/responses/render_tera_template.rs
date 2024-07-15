use serde::Serialize;
use tera::{Context, Result, Tera};

pub fn render_tera_template(content: &str, props: &impl Serialize) -> Result<String> {
  let mut tera = Tera::default();
  tera.add_raw_template("root_layout", content)?;
  tera.render("root_layout", &Context::from_serialize(props)?)
}
