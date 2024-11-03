#![allow(unused_variables)]

// Just a few examples of useful traits

struct Salary(u32);

impl Default for Salary {
    fn default() -> Self {
        // When you implement this one, rust-analyzer may hint about perhaps doing this automatically for you 😏🪄
        // TODO: implement default salary
        // Hint: started from the bottom, now you're here
        todo!()
    }
}

fn t_me<T>() -> T {
    // Hint: maybe we need to know more about T before we can return one?
    todo!("Implement me by changing what we know about T. Perhaps there is some default behavior?")
}

/// Not really a duplicator, but it gives you another default value!
fn duplicate<T>(t: T) -> (T, T) {
    // Hint: maybe we need to know more about T before we can return two? We only have one! If only we had a Clone
    todo!()
}

/// No one really gets over 255 years old, right?
/// This is how we can automatically derive trait implementations for types.
/// In this case we want to compare two ages for equality, and also
/// compare two ages for who is oldest (or youngest).
/// Read more about partial and total ordering (and equality) here:
/// <https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html>
/// There's also `Eq` and `Ord` for total equality and ordering, respectively.
#[derive(PartialEq, PartialOrd)]
struct Age(u8);

fn a_is_older_than_b(a: Age, b: Age) -> bool {
    a > b
}

fn printable<T: std::fmt::Display>(t: T) {
    // Types that are printable (with a proper representation),
    // either to the console or just represented as a string,
    // implement the trait Display
    let string_representation: String = format!("{t}");
    println!("{t}");
}

fn debug_printable<T: std::fmt::Debug>(t: T) {
    let debug_representation: String = format!("{t:?}");
    let debug_representation_pretty_printed: String = format!("{t:#?}");

    // The dbg-macro is really useful for cowboy-debugging, ehhem,
    // it prints the value of the item pretty-printed along with line number
    // and finally returns the value so it can be used further
    let t = dbg!(t);
}

// Example trait for a machine that inputs one thing and outputs another,
// but there may be several outputs for every thing you implement the trait for.
trait MyMachine {
    // This is an associated type, the implementor decides what it should be
    type Output;

    // The associated type can be referenced later
    fn churn(self) -> Self::Output;
}

struct Cream;

struct Butter;

impl MyMachine for Cream {
    type Output = Butter;
    fn churn(self) -> Butter {
        Butter
    }
}

// Associated types can be referenced like such                  👇
// This is an alias for Butter 👇
type MachineOutputAliasCanBeReferencedLike = <Cream as MyMachine>::Output;

/// Serialization and deserialization is idiomatically done through a
/// community-driven framework called `serde`.
/// It is driven by an AST-style core, so if your type can be represented
/// with this AST, it can be serialized and deserialized into over 20 formats.
///
/// The most common way to use it is to derive the corresponding
/// trait onto your type. This will compile-time generate the trait
/// implementations necessary to serialize your type. If it is not
/// possible to serialize your type automatically it will fail to compile.
#[derive(serde::Serialize)]
struct Participant {
    name: String,
    is_participating: bool,
    /// Can even serialize weird things, such as a vector of units
    /// (a zero-sized-type)
    belongings: Vec<()>,
    // Though cannot serialize some weird types, such as never/Infallible/!.
    // does_not_work: Option<std::convert::Infallible>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn started_from_the_bottom_and_were_still_there() {
        assert_eq!(Salary::default().0, Salary(0).0);
    }

    #[test]
    fn give_me_a_salary() {
        assert_eq!(t_me::<Salary>().0, Salary(0).0);
    }

    #[test]
    fn duplicates() {
        impl Clone for Salary {
            fn clone(&self) -> Self {
                Self(self.0)
            }
        }
        let (ka, ching) = duplicate(Salary(123));
        assert_eq!(ka.0, 123);
        assert_eq!(ching.0, 123);
    }
}
