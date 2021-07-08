use handlebars::Handlebars;

use warp::{ Filter, Reply, Rejection };

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
	let mut handlebars = Handlebars::new();

	if let Err(error) = register_templates(&mut handlebars) {
		log::error!("Error while registering Handlebars templates: {}", error);
	}
	
	let single_page_app = warp::fs::dir("spa");

	return single_page_app;
}

fn register_templates(handlebars: &mut Handlebars) -> Result<(), anyhow::Error> {
	handlebars.register_template_file("main", "./views/main.hbs")?;
	handlebars.register_template_file("layouts", "./views/layouts.hbs")?;
	Ok(())
}