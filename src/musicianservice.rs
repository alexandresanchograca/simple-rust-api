use serde::{ser::SerializeStruct, Serialize};

pub struct Musician{
    id: u32,
    name: String,
    famous_works : Vec<&'static str>,
}

impl Serialize for Musician {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut state = serializer.serialize_struct("Musician", 3)?;
        state.serialize_field("id", &self.id);
        state.serialize_field("name", &self.name);
        state.serialize_field("famous_works", &self.famous_works);
        state.end()
    }
}

impl Musician {

    pub fn get_musicians_json() -> String {
       let objects = Musician::get_musicians();
       serde_json::to_string(&objects).unwrap()
    }

    fn get_musicians() -> Vec<Musician> {
        //Initialize Mock Painters Data

        let musician_vec : Vec<Musician> = vec![
            Musician {
                id: 1,
                name: String::from("Wolfgang Amadeus Mozart"),
                famous_works: vec![
                    "Symphony No. 40 in G minor",
                    "Eine kleine Nachtmusik",
                    "Piano Sonata No. 11 in A major (K. 331)"
                  ],
            },
            Musician {
                id: 2,
                name: String::from("Ludwig van Beethoven"),
                famous_works: vec![
                    "Symphony No. 9 in D minor (Choral)",
                    "Für Elise",
                    "Piano Sonata No. 14 in C-sharp minor (Moonlight Sonata)"
                  ],
            },
            Musician {
                id: 3,
                name: String::from("Johann Sebastian Bach"),
                famous_works: vec![
                    "Brandenburg Concertos",
                    "The Well-Tempered Clavier",
                    "Matthäus-Passion (St. Matthew Passion)"
                  ],
            },
        ];

        musician_vec
    }
}


