#[derive(serde::Deserialize, PartialEq, Debug)]
pub(crate) struct ParsedPerson {
    name: String,
    is_alive: bool,
    is_bald: Option<bool>,
}

impl ParsedPerson {
    fn new(name: String, is_alive: bool, is_bald: Option<bool>) -> Self {
        Self {
            name,
            is_alive,
            is_bald,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Person {
    name: String,
    life_status: LifeStatus,
    hair_style: Option<HairStyle>,
}

#[derive(PartialEq, Debug)]
enum LifeStatus {
    Alive,
    Dead,
}

#[derive(PartialEq, Debug)]
enum HairStyle {
    Bald,
    Hairy,
}

// From and Into are traits to convert to and from things
// and implementing From gives a blanket implementation to Into
// which goes the other way around!

impl From<ParsedPerson> for Person {
    fn from(person: ParsedPerson) -> Self {
        Person {
            name: person.name,
            // This is not super clean, but I wouldn't implement `From<bool> for LifeStatus`
            // because a boolean itself doesn't really indicate whether we are
            // checking for liveness or deadness
            life_status: if person.is_alive {
                LifeStatus::Alive
            } else {
                LifeStatus::Dead
            },
            // The hair-style example is also bad but I added it so you could
            // see how to deserialize optional values from JSON to Rust structs.
            hair_style: person.is_bald.map(|is_bald| {
                if is_bald {
                    HairStyle::Bald
                } else {
                    HairStyle::Hairy
                }
            }),
        }
    }
}
