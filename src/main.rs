#[macro_use]
extern crate rocket;

use rocket::http::ContentType;
use rocket::response::content::Html;
use rust_embed::RustEmbed;

use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(RustEmbed)]
#[folder = "$EMBEDDED_FILES_PATH"]
struct Asset;

#[get("/")]
fn index() -> Option<Html<Cow<'static, [u8]>>> {
	let asset = Asset::get("index.html")?;
	Some(Html(asset.data))
}

#[get("/<file..>")]
fn serve_embedded_file(file: PathBuf) -> Option<(ContentType, Cow<'static, [u8]>)> {
	let filename = file.display().to_string();
	let asset = Asset::get(&filename)?;
	let content_type = file
		.extension()
		.and_then(OsStr::to_str)
		.and_then(ContentType::from_extension)
		.unwrap_or(ContentType::Bytes);

	Some((content_type, asset.data))
}

#[rocket::launch]
fn rocket() -> _ {
	let yaml = clap::load_yaml!("cli.yaml");
	let matches = clap::App::from(yaml).get_matches();

	// we can safely use unwrap here since yaml file includes default values
	let port = matches.value_of_t::<usize>("port").unwrap();
	
	let figment = rocket::Config::figment().merge(("port", port));
	rocket::custom(figment).mount("/", routes![index, serve_embedded_file])
}