// pub(crate) struct Salary(u32);

// impl Default for Salary {
//     fn default() -> Self {
//         Salary(0)
//     }
// }

// You can do like this 👆
// But idiomatic Rust is to derive `Default` when
// your type only needs the "classic" defaults,
// such as zero for numbers, etc.
// PartialEq let's you compare types so e.g.
// you can write:
// `Salary(0) < Salary(123)`
// instead of
// `Salary(0).0 < Salary(123).0`

#[derive(Default, PartialEq)]
pub(crate) struct Salary(pub(crate) u32);

pub(crate) fn t_me<T: Default>() -> T {
    T::default()
    // Or
    // Default::default()
}

// OR

// pub(crate) fn t_me<T>() -> T
// where
//     T: Default,
// {
//     T::default()
// }

// Clone is really useful when you need to copy two values.
// Copy is a trait similar to clone, but implicitly calls `.clone` when
// you would move a value

pub(crate) fn duplicate<T: Clone>(t: T) -> (T, T) {
    (t.clone(), t)
}
