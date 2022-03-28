use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Title {
    pub _id: String,
    pub name: String,
    pub published_year: i32,
    pub appeared_characters: Vec<String>,
}

use super::characters::Character;

#[graphql_object(description = "東方Projectの作品")]
impl Title {
    fn id(&self) -> &str {
        return &self._id;
    }
    fn name(&self) -> &str {
        return &self.name;
    }
    fn published_year(&self) -> i32 {
        return self.published_year;
    }
    fn appeared_characters(&self) -> Vec<Character> {
        return super::characters::get_characters()
            .iter()
            .filter(|c| self.appeared_characters.contains(&c.name))
            .cloned()
            .collect();
    }
}

use memoise::memoise;
use serde_yaml;
#[memoise(true == true)]
fn get_titles_yaml() -> Vec<Title> {
    let titles = std::fs::read_to_string("./titles.yml").map(|titles_string| {
        let y: Result<Vec<Title>, _> = serde_yaml::from_str(&titles_string);
        return y.unwrap();
    });

    return titles.unwrap();
}

pub fn get_titles() -> Vec<Title> {
    /* If you need post processing, write it here */
    return get_titles_yaml();
}
