use juniper::graphql_object;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Character {
    pub _id: String,
    pub name: String,
    pub human_factor: f64,
}

use super::titles::Title;

#[graphql_object(description = "東方Projectの登場人物")]
impl Character {
    fn id(&self) -> &str {
        return &self._id;
    }
    fn name(&self) -> &str {
        return &self.name;
    }
    fn human_factor(&self) -> f64 {
        return self.human_factor;
    }
    fn appeared_in_title(&self) -> Vec<Title> {
        return super::titles::get_titles()
            .iter()
            .filter(|t| t.appeared_characters.contains(&self.name))
            .cloned()
            .collect();
    }
}

use memoise::memoise;
use serde_yaml;
#[memoise(true == true)]
fn get_characters_yaml() -> Vec<Character> {
    let characters = std::fs::read_to_string("./characters.yml").map(|characters_string| {
        let y: Result<Vec<Character>, _> = serde_yaml::from_str(&characters_string);
        return y.unwrap();
    });

    return characters.unwrap();
}

pub fn get_characters() -> Vec<Character> {
    /* If you need post processing, write it here */
    return get_characters_yaml();
}
