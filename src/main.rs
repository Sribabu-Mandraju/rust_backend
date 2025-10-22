use actix_web::{App, HttpServer, Responder, HttpResponse, get, web};
use actix_files::Files;
mod models;
mod services;
mod routes;
use models::Todo;
use routes::evm::{generate_keypair, send_ether, sign_message, verify_message};


// Existing endpoints
#[get("hello/{firstname}")]
async fn home(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}")
}

#[get("/todos")] 
async fn create_todo(todo: web::Json<Todo>) -> impl Responder {
    HttpResponse::Ok().body(serde_json::to_string(&todo).unwrap())
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body(r#"
<!DOCTYPE html>
<html>
<head>
    <title>EVM Backend API</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #f5f5f5; }
        .container { max-width: 800px; margin: 0 auto; background: white; padding: 30px; border-radius: 10px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #333; text-align: center; }
        .endpoint { margin: 20px 0; padding: 15px; background: #f8f9fa; border-left: 4px solid #007bff; }
        .method { background: #28a745; color: white; padding: 4px 8px; border-radius: 4px; font-size: 12px; font-weight: bold; }
        .path { font-family: monospace; color: #666; margin-left: 10px; }
        .btn { background: #007bff; color: white; padding: 10px 20px; text-decoration: none; border-radius: 5px; display: inline-block; margin-top: 20px; }
        .btn:hover { background: #0056b3; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🚀 EVM Backend API</h1>
        <p>Welcome to the Ethereum Virtual Machine Backend API. This server provides endpoints for EVM operations.</p>
        
        <div class="endpoint">
            <span class="method">GET</span><span class="path">/hello/{name}</span>
            <p>Simple greeting endpoint</p>
        </div>
        
        <div class="endpoint">
            <span class="method">POST</span><span class="path">/evm/generate-keypair</span>
            <p>Generate a new Ethereum keypair</p>
        </div>
        
        <div class="endpoint">
            <span class="method">POST</span><span class="path">/evm/send-ether</span>
            <p>Send ether on testnet with balance validation</p>
        </div>
        
        <div class="endpoint">
            <span class="method">POST</span><span class="path">/evm/sign-message</span>
            <p>Sign a message with a private key</p>
        </div>
        
        <div class="endpoint">
            <span class="method">POST</span><span class="path">/evm/verify-message</span>
            <p>Verify a message signature</p>
        </div>
        
        <a href="/static/index.html" class="btn">🎯 Open Interactive Interface</a>
    </div>
</body>
</html>
    "#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(root)
        .service(home)
        .service(create_todo)
        .service(generate_keypair)
        .service(send_ether)
        .service(sign_message)
        .service(verify_message)
        .service(Files::new("/static", "./static").show_files_listing())
    )
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}