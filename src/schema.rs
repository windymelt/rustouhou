use juniper::FieldResult;
use juniper::{EmptyMutation, EmptySubscription, RootNode};

mod characters;
mod titles;

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn character(_id: String) -> FieldResult<Option<characters::Character>> {
        let chars = characters::get_characters();
        let char = chars.iter().find(|c| c._id == _id).map(|c| c.clone());
        return Ok(char);
    }
    fn character_with_name(name: String) -> FieldResult<Option<characters::Character>> {
        let chars = characters::get_characters();
        let char = chars.iter().find(|c| c.name == name).map(|c| c.clone());
        return Ok(char);
    }
    fn title(_id: String) -> FieldResult<Option<titles::Title>> {
        let titles = titles::get_titles();
        let title = titles.iter().find(|t| t._id == _id).map(|t| t.clone());
        return Ok(title);
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
