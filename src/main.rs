use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer};
use std::fs;

#[get("/")]
async fn index() -> HttpResponse {
	let index = fs::read_to_string("static/index.html").unwrap();
	HttpResponse::Ok().body(index)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	simple_logger::init_with_level(log::Level::Info).unwrap();

	let port = 8080;
	let ip_addr = "0.0.0.0";

	println!("Starting server on port: {port}");
	println!("http://localhost:{port}");

	HttpServer::new(|| {
		App::new()
			.service(index)
			// Это чтобы сервер грузил стили из папки static НЕТРОГАТЬ!!!!
			.service(Files::new("/", "./static").show_files_listing())
	})
	.bind((ip_addr, port))?
	.workers(2)
	.run()
	.await
}
