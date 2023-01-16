// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use std::{ops::Rem, fmt::Debug};

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T>
where
    T: ToString,
{
    function: Box<dyn Fn(T) -> bool>,
    word: String, 
}

impl<T: ToString> Matcher<T> {
    pub fn new<F: Fn(T) -> bool + 'static>(function: F, word: &str) -> Matcher<T> {
        Self { function: Box::new(function), word: word.to_string() }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy< T: ToString>{
    matchers: Vec<Matcher<T>>,
}

impl<T: ToString + Clone + Debug + 'static> Fizzy<T> {
    pub fn new() -> Self {
        Self {matchers: Vec::new()}
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply(self, iter: impl Iterator<Item = T> ) -> impl Iterator<Item = String> {
        iter.map(move |t| {
            let mut result = String::new();
            for m in &self.matchers {
                if (m.function)(t.clone()) {
                    result += &m.word;
                }
            }

            if result.is_empty() {t.to_string()}
            else {result}
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T: ToString + Clone + From<u8> + Rem<Output = T> + Debug + 'static>() -> Fizzy<T> 
where
<T as Rem>::Output: PartialEq
{
    Fizzy::new()
        .add_matcher(Matcher::new(|x: T| x % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|x: T| x % T::from(5) == T::from(0), "buzz"))
}
