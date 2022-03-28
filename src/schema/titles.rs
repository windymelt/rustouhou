use serde::Deserialize;

#[derive(Clone)]
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

#[derive(Deserialize, Clone)]
pub struct YamlTitle {
    pub id: String,
    pub name: String,
    pub published_year: i32,
    pub appeared_characters: Vec<String>,
}

use memoise::memoise;
use serde_yaml;
#[memoise(true == true)]
fn get_titles_yaml() -> Vec<YamlTitle> {
    let titles = std::fs::read_to_string("./titles.yml").map(|titles_string| {
        let y: Result<Vec<YamlTitle>, _> = serde_yaml::from_str(&titles_string);
        return y.unwrap();
    });

    return titles.unwrap();
}

pub fn get_titles() -> Vec<Title> {
    let titles: Vec<Title> = get_titles_yaml()
        .iter()
        .map(|t| Title {
            _id: t.id.clone(),
            name: t.name.clone(),
            published_year: t.published_year,
            appeared_characters: t.appeared_characters.clone(),
        })
        .collect();

    return titles;
}
