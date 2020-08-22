use actix_web::{middleware, web, App, HttpRequest, HttpServer, Responder};
use juniper::FieldResult;
use juniper::GraphQLInputObject;
use juniper::GraphQLObject;
use juniper::RootNode;
use std::io::{stdout, Error, Write};
use uuid::Uuid;
extern crate actix;

#[derive(GraphQLObject)]
#[graphql(description = "Service that is monitored")]
struct Service {
    id: Uuid,
    name: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct Ping {
    id: Uuid,
    service: Service,
    start_time: i32,
    response_time: i32,
    status_code: i32,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A sinmple ping request")]
struct PingRquest {
    name: String,
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn allPing() -> FieldResult<Ping> {
        Ok(Ping {
            id: Uuid::new_v4(),
            service: Service {
                id: Uuid::new_v4(),
                name: "Main Service".to_owned(),
            },
            start_time: 1231233,
            response_time: 12324,
            status_code: 200,
        })
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
    async fn create_ping(ping_request: PingRquest) -> FieldResult<Ping> {
        Ok(Ping {
            id: Uuid::new_v4(),
            service: Service {
                id: Uuid::new_v4(),
                name: "Main Service".to_owned(),
            },
            start_time: 1231233,
            response_time: 12324,
            status_code: 200,
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub async fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
