use std::{io::Result, sync::Arc};

use actix_web::{App, HttpServer, Responder, HttpResponse, route, web::{Data, self}};
use juniper::http::GraphQLRequest;
use schema::create_schema;

use crate::schema::{Schema, DatabaseContext};

pub mod schema;
pub mod query;
pub mod mutation;

#[actix_web::main]
async fn main() -> Result<()> {
    let schema = Arc::new(create_schema());
    let database = Arc::new(DatabaseContext::new());
    let app = move || {
        App::new()
        .app_data(Data::from(schema.clone()))
        .app_data(Data::from(database.clone()))
        .service(graphql)
    };

    HttpServer::new(app).bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

#[route("/graphql", method = "POST")]
async fn graphql(schema: web::Data<Schema>, data: web::Json<GraphQLRequest>, context: web::Data<DatabaseContext>) -> impl Responder {
    // here the context is reinitialized at each request, we don't want that
    let response = data.execute(&schema, &context).await;
    HttpResponse::Ok().json(response)
}
