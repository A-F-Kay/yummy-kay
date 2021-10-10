use actix_web::{get, web, App, HttpServer, Responder};

type HostInfo = (String, u16);

pub struct Server {
    host_info: HostInfo,
}

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

impl Server {
    pub fn new(host_info: HostInfo) -> Self {
        Server { host_info }
    }

    pub async fn start(&self) -> std::io::Result<()> {
        HttpServer::new(|| App::new().service(index))
            .bind(&self.host_info)?
            .run()
            .await
    }
}
