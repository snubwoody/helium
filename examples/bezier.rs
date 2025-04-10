use helium::colors::BLUE;
use ruby::{Bezier, Color, Rect, Text};

#[tokio::main]
async fn main() -> ruby::Result<()> {
	let app = ruby::App::new()?;
	
	app.run(move |r|{
		let b = Bezier::new(200.0, 200.0).color(BLUE);
		r.draw_bezier(b);
	}).await?;

	Ok(())
}
