use juniper::{EmptyMutation, EmptySubscription, RootNode};
use juniper::{FieldError, FieldResult};

mod characters;

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn character(_id: String) -> FieldResult<characters::Character> {
        let chars = characters::get_characters();
        let char = chars.iter().find(|c| c.id == _id).map(|c| c.clone());
        return char.ok_or(FieldError::from("Character not found"));
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
