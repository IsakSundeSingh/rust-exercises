#[derive(serde::Deserialize, PartialEq, Debug)]
struct ParsedPerson {
    // TODO: define me
}

impl ParsedPerson {
    fn new(name: String, is_alive: bool, is_bald: Option<bool>) -> Self {
        todo!()
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
        todo!("implement this to just feel how to use From")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn albert() -> ParsedPerson {
        ParsedPerson::new("Albert Einstein".into(), false, None)
    }

    fn jeff() -> ParsedPerson {
        ParsedPerson::new("Jeff Bezos".into(), true, Some(true))
    }

    #[test]
    fn can_be_deserialized_from_json() {
        // include_str embeds the string as part of the executable
        let json = include_str!("../inputs/people_inputs.json");
        let deserialized: Vec<ParsedPerson> =
            serde_json::from_str(json).expect("Deserialization failed");

        assert_eq!(deserialized, vec![albert(), jeff(),])
    }

    #[test]
    fn use_from_to_convert_a_type_with_primitives_into_a_type_with_better_naming() {
        let parsed_person = albert();

        assert_eq!(
            Person::from(parsed_person),
            Person {
                name: "Albert Einstein".into(),
                life_status: LifeStatus::Dead,
                hair_style: None
            }
        );
    }
}
