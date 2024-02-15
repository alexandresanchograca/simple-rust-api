use serde::{ser::SerializeStruct, Serialize};

pub struct Painter{
    id: u32,
    name: String,
    famous_works : Vec<&'static str>,
}

impl Serialize for Painter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut state = serializer.serialize_struct("Painter", 3)?;
        state.serialize_field("id", &self.id);
        state.serialize_field("name", &self.name);
        state.serialize_field("famous_works", &self.famous_works);
        state.end()
    }
}

impl Painter {

    pub fn get_painters_json() -> String {
       let objects = Painter::get_painters();
       serde_json::to_string(&objects).unwrap()
    }

    fn get_painters() -> Vec<Painter> {
        //Initialize Mock Painters Data

        let painter_vec : Vec<Painter> = vec![
            Painter {
                id: 1,
                name: String::from("Vincent van Gogh"),
                famous_works: vec!["The Starry Night", "Sunflowers", "Irises"],
            },
            Painter {
                id: 2,
                name: String::from("Leonardo da Vinci"),
                famous_works: vec!["The Starry Night", "Sunflowers", "Irises"],
            },
            Painter {
                id: 3,
                name: String::from("Pablo Picasso"),
                famous_works: vec![
                    "Guernica",
                    "Les Demoiselles d'Avignon",
                    "The Weeping Woman",
                ],
            },
        ];

        painter_vec
    }
}


