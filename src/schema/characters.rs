use juniper::GraphQLObject;
use serde::Deserialize;

#[derive(GraphQLObject, Deserialize, Clone)]
#[graphql(description = "東方Projectの登場人物")]
pub struct Character {
    pub id: String,
    pub name: String,
    pub human_factor: f64,
}

use serde_yaml;
pub fn get_characters() -> Vec<Character> {
    let characters = std::fs::read_to_string("./characters.yml").map(|characters_string| {
        let y: Result<Vec<Character>, _> = serde_yaml::from_str(&characters_string);
        return y.unwrap();
    });

    return characters.unwrap();
}
