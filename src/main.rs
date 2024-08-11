use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct PlistQuery {
    bundleid: String,
    name: String,
    version: String,
}

static PLIST_TEMPLATE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>items</key>
    <array>
        <dict>
            <key>assets</key>
            <array>
                <dict>
                    <key>kind</key>
                    <string>software-package</string>
                    <key>url</key>
                    <string>https://localhost.direct:9090/tempsigned.ipa</string>
                </dict>
            </array>
            <key>metadata</key>
            <dict>
                <key>bundle-identifier</key>
                <string>{bundleid}</string>
                <key>bundle-version</key>
                <string>{version}</string>
                <key>kind</key>
                <string>software</string>
                <key>title</key>
                <string>{name}</string>
            </dict>
        </dict>
    </array>
</dict>
</plist>"#;

async fn generate_plist(query: web::Query<PlistQuery>) -> impl Responder {
    let plist_xml = PLIST_TEMPLATE
        .replace("{bundleid}", &query.bundleid)
        .replace("{version}", &query.version)
        .replace("{name}", &query.name);

    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(plist_xml)
}

async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/genPlist", web::get().to(generate_plist))
            .route("/ping", web::get().to(ping))
    })
    .bind("0.0.0.0:10000")?
    .run()
    .await
}
