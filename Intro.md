---
marp: true
theme: gaia
transition: melt
---

<!-- _class: lead -->
<!-- _color: orange -->

# Rust encapsulation, generics and polymorphism

---

# Agenda

- Methods vs functions
- Encapsulation with modules and methods
- Generics
- Traits (polymorphism)
- Iterators and other useful traits

---

# Functions and methods

- Functions and methods are similar, but their usages are different.
- A function is freestanding and can be called on its on: `my_function(my_arg, other_arg)`
- A method differs in that it is usually invoked on a value, where the value is passed as an argument: `my_arg.my_method(other_arg)`
- Rust also allows calling on methods as functions: `Arg::my_method(my_arg, other_arg)`, assuming `my_arg: Arg`.

---

# Defining methods

To define a method in Rust, use an `impl`-block on the type:

```rust
struct Square {
    side_length: u32,
}
impl Square {
    fn area(self: &Square) -> u32 {
        self.side_length * self.side_length
    }
}
let square = Square { side_length: 2 };
println!("{}", square.area());
// Prints: 4
```

---

# Defining methods

Methods must have a _receiver_ that accepts `self` in the first parameter to be able to be invoked like `square.area()`

```rust
impl Square {
    fn area(self: &Square) -> u32 {
        self.side_length * self.side_length
    }
    // is equivalent to
    fn area(self: &Self) -> u32 { ... }
    // is equivalent to the following, which is idiomatic
    fn area(&self) -> u32 { ... }
}
```

---

# Different receivers in methods

A few types of receivers exist, but the following are the most common

```rust
impl Square {
    fn erase(self) {
        // Takes ownership and _moves_ self into the function.
        // self can no longer be used
    }
    fn read(&self) {
        println!("Square with length {}", self.side_length)
    }
    fn enlarge(&mut self) {
        self.side_length *= 2;
    }
}
```

---

# Calling methods

```rust
let square = Square { side_length: 2 };

square.read();
Square::read(&square);

let mut square = square;
square.enlarge();
Square::enlarge(&mut square);

square.erase();
// Create a new square since the previous was dropped
Square::erase(Square { side_length: 2 });
```

---

# Modules

A file is a module in Rust. You can also make inline modules by writing `mod my_module { /* content of module here */ }`
If you have a program `src/main.rs` and want to create a module `src/square.rs`, you need to declare in `src/main.rs` that `square.rs` exists by using `mod square;` at the top of the main file.

```rust
// in src/square.rs
struct Square { side_length: u32 }
```

```rust
// in src/main.rs
mod square;
// now you can use square::Square
```

---

# Modules

- Modules are a way to colocate and encapsulate code to control _scope_ and _privacy_.
- As you would create a private field and a public property in classes in OOP, you can create private fields and public methods in Rust.
- Modules also allow controlling private and public functions
- By default, code within a module is private from its parent modules by default. E.g. `mod private { fn foo() {} }` inside `mod main` means the `main`-module cannot call `foo` since it is private.
- Items can be made public by adding a visibility modifier: `pub fn foo() {}`

---

# Visibility modifiers

Several variants of the visibility modifiers exist, but most of the time, you only need and want `pub` or not. Try _not_ to do this:

```rust
mod main_module {
    mod some_mod {
        mod private {
            fn private_function() {}
            pub fn public_function_for_those_with_access_to_private() {}
            pub(super) fn public_in_parent_module() {}
            pub(in crate::main_module::some_mod) fn also_public_in_parent() {}
        }
    }
}
```

---

# Encapsulation

Using modules is a nice way to encapsulate code and invariants and not allow users to break them.

Store state internally in types and provide methods to access and use the data without breaking those invariants.

---

# Encapsulation

```rust
mod time {
    struct Years(u32); struct Days(u32);
    impl Years {
        pub fn new(years: u32) -> Self { Self(years) }
        pub fn to_days(&self) -> Days {
            Days(self.0 * 365)
        }
    }
    impl Days {
        pub fn new(days: u32) -> Self { Self(days) }
        pub fn to_years(&self) -> Years {
            Years(self.0 / 365)
        }
    }
    fn is_adult(age: &Years) -> bool {
        age.0 >= 18
    }
}
```

---

# New types

An especially useful concept is _new types_, which wrap around primitive types and may or may not add invariants. E.g. you wouldn't pass around `String` when you could have `Email` with invariants:

```rust
struct Email(String);
impl Email {
    fn new(email: String) -> Result<Self, EmailParseError> {
        let parsed = parse_email(&email);
        match parsed {
            Ok(parsed_email) => Ok(Self(parsed_email)),
            Err(parse_error) => Err(parse_error),
        }
    }
}
```

---

# Generics

To avoid writing the same code multiple times, Rust has _generics_. That is, an item can be _generic_ over a type.

```rust
fn function<T>(t: T) -> T { return t; }
```

`function` is generic over `T`, which means it knows nothing about `T`. It _cannot_ be printed, created new values from, copied or cloned, nor compared with. It can only be returned or _moved_ (e.g. dropped):

```rust
fn function<T>(t: T) {
    drop(t)
}
```

---

# How to use generics: Traits

Generic types are not that useful without something more.
Rust uses `traits` to give behavior to types, which can be used on generic types as well.

```rust
/// Allows something to be printed
trait Printable {
    fn print(&self);
}
struct Square { side_length: u32 }
impl Printable for Square {
    fn print(&self) {
        println!("Square with side_length: {}", self.side_length);
    }
}
```

---

# Generics and traits

```rust
// Long form
fn print<T>(t: T) where T: Printable {
    t.print();
}
// or more commonly
fn print<T: Printable>(t: T) {
    t.print();
}
// or if you only want Printable's behavior
// accept something that implements Printable
fn print(t: impl Printable) {
    t.print();
}
```

---

# More on traits

- Traits only defines behavior, implementing them on a type gives them values to work on
- Traits can be empty, `trait Marker {}`, which are called marker traits. E.g. if you don't need any behavior, you just want to mark something, such as `Sized` meaning a type has a known size.
- The standard library is often implemented using traits for common functionality, such as:
  - types displayable to users (`Display`)
  - types that can be printed for debugging (`Debug`)
  - types that are comparable for equality (`PartialEq`), etc.

---

# `Clone` and `Copy` traits

`Clone` and `Copy` are traits that declare whether a type can be cloned. `Copy` is a _supertrait_ of `Clone`, meaning to be `Copy` you need to be `Clone` as well (written as `Copy: Clone`).

Semantically, `Clone` means clonable, and `Copy` means cheaply clonable, and for a type that is `Copy` the compiler implicitly calls `.clone()` when the value is moved

```rust
fn can_clone<T: Clone>(t: &T) -> (T, T) {
    (t.clone(), t.clone())
}
```

---

# `Clone` and `Copy`, continued

Primitives such as numbers are `Copy`, but `String`s are `Clone`, because a string has a varying size and can be a byte or a gigabyte.

```rust
fn consumes<T>(t: T) {}
let x: i32 = 2;
consumes(x); // consumes would _move_ x so the compiler copies it
println!("{x}"); // x can still be used
let y = String::from("hello");
consumes(y); // String is !Copy, but is Clone, so the value is moved and consumed
// printing y here fails to compile
#[derive(Clone, Copy)] // Making Thing Clone and Copy makes it like numbers
struct Thing;
let z = Thing;
consumes(z);
consumes(z); // z is reusable because it was copied
```

---

# Converting `Into<T>` and `From<U>`

Converting between values is standardized with the `Into`- and `From`-traits.

```rust
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}
pub trait Into<T>: Sized {
    fn into(self) -> T;
}
fn accepts_string(x: String) {}
let value: &'static str = "hello";
accepts_string(value.into());
let y: String = String::from(value);
```

---

# `Iterator`s

Computing on multiple values is a really common action. Rust has generalized it using the `Iterator`-trait. It is also what `for x in iter` uses.

```rust
pub trait Iterator {
    /// The type of the elements being iterated over.
    type Item;

    /// Advances the iterator and returns the next value.
    ///
    /// Returns [`None`] when iteration is finished.
    fn next(&mut self) -> Option<Self::Item>;

```

---

# Implementing `Iterator`

An endless iterator that produces odd numbers (until `u32` overflows):

```rust
struct OddIterator {
    current: u32,
}
impl Iterator for OddIterator {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current;
        self.current += 2;
        Some(value)
    }
}
```

---

# Benefits of `Iterator`

<style scoped>
section {
    font-size: 1.9rem;
}
</style>

Implementing a single function gives access to 72+ methods, since they are implemented using only `next`:

`advance_by`, `all`, `any`, `array_chunks`, `by_ref`, `chain`, `cloned`, `cmp`, `cmp_by`, `collect`, `collect_into`, `copied`, `count`, `cycle`, `enumerate`, `eq`, `eq_by`, `filter`, `filter_map`, `find`, `find_map`, `flat_map`, `flatten`, `fold`, `for_each`, `fuse`, `ge`, `gt`, `inspect`, `intersperse`, `intersperse_with`, `is_partitioned`, `is_sorted`, `is_sorted_by`, `is_sorted_by_key`, `last`, `le`, `lt`, `map`, `map_while`, `map_windows`, `max`, `max_by`, `max_by_key`, `min`, `min_by`, `min_by_key`, `ne`, `next_chunk`, `nth`, `partial_cmp`, `partial_cmp_by`, `partition`, `partition_in_place`, `peekable`, `position`, `product`, `reduce`, `rev`, `rposition`, `scan`, `size_hint`, `skip`, `skip_while`, `step_by`, `sum`, `take`, `take_while`, `try_collect`, `try_find`, `try_fold`, `try_for_each`, `try_reduce`, `unzip`, and `zip`

---

# Lazy `Iterator`s

Notice that Iterators are lazy, so the do _no_ work unless `.next()` is called, such as each iteration of a `for`-loop, or creating a collection from an iterator:

```rust
// Is empty, because OddIterator only gives odd numbers
// and our filter filters only even numbers
let only_evens: Vec<u32> = OddIterator { current: 1 }.filter(|x| x % 2 == 0).collect();
```

Nor do `Iterator`s allocate unless specified, so they are very efficient to use compared to many other languages where lazyness is opt-in. E.g. Kotlin's `Sequence<T>` instead of `List<T>`, C#'s `IEnumerable<T>` instead of `List<T>`, and others.
