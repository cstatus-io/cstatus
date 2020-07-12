use juniper::FieldResult;
use juniper::GraphQLObject;
use juniper::RootNode;
use uuid::Uuid;

#[derive(GraphQLObject)]
#[graphql(description = "Service that is monitored")]
struct Service {
    id: Uuid,
    name: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Ping {
    id: Uuid,
    service: Service,
    start_time: i32,
    response_time: i32,
    status_code: i32,
}

// #[derive(GraphQLInputObject)]
// #[graphql(description = "A humanoid creature in the Star Wars universe")]
// struct NewHuman {
//     name: String,
//     appears_in: Vec<Episode>,
//     home_planet: String,
// }

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
    // fn create_human(new_human: NewHuman) -> FieldResult<Human> {
    //     Ok(Human {
    //         id: "1234".to_owned(),
    //         name: new_human.name,
    //         appears_in: new_human.appears_in,
    //         home_planet: new_human.home_planet,
    //     })
    // }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
