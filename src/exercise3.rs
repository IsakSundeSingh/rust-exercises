#![allow(unused_variables)]

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
    todo!()
}

/// Not really a duplicator, but it gives you another default value!
fn duplicate<T>(t: T) -> (T, T) {
    // Hint: maybe we need to know more about T before we can return two? We only have one! If only we had a Clone
    todo!()
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
