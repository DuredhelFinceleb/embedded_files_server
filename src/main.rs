#[macro_use] 
extern crate rocket;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::{ContentType, Status};

use rust_embed::{RustEmbed, EmbeddedFile};

use std::ffi::OsStr;
use std::io::Cursor;
use std::path::PathBuf;

#[derive(RustEmbed)]
#[folder = "test_folder"]
struct Asset;

struct MyFile {
	path_buf: Option<PathBuf>,
	embedded: EmbeddedFile,
	content_type: Option<ContentType>,
}

impl<'r> Responder<'r, 'static> for MyFile {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
		let d = self.embedded;

		let content_type = self.path_buf.map_or_else (
			|| self.content_type.ok_or_else(|| Status::new(400)),
			|file| {
				let ext = file
					.as_path()
					.extension()
					.and_then(OsStr::to_str)
					.ok_or_else(|| Status::new(400))?;
				ContentType::from_extension(ext).ok_or_else(|| Status::new(400))
			},
		)?;
		
		Response::build().header(content_type).sized_body(d.data.len(), Cursor::new(d.data)).ok()
    }
}

#[get("/")]
fn index() -> Result<MyFile, Status> {
	Asset::get("index.html").map_or_else(
		|| Err(Status::NotFound),
		|d| Ok(MyFile { path_buf: None, embedded: d, content_type: Some(ContentType::HTML) }),
	)
}

#[get("/<file..>")]
fn serve_embedded_file(file: PathBuf) -> Result<MyFile, Status> {
	let filename = file.display().to_string();
	Asset::get(&filename).map_or_else(
		|| Err(Status::NotFound),
		|d| Ok(MyFile { path_buf: Some(file), embedded: d, content_type: None }),
	)
}

#[launch]
fn rocket() -> _ {
	let yaml = clap::load_yaml!("cli.yaml");
	let matches = clap::App::from(yaml).get_matches();

	// we can safely use unwrap here since yaml file includes default values
	// let embedded_files_folder = matches.value_of("path").unwrap();
	let port = matches.value_of_t::<usize>("port").unwrap();
	
	let figment = rocket::Config::figment().merge(("port", port));
	rocket::custom(figment).mount("/", routes![index, serve_embedded_file])
}