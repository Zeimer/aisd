pub mod bst;

pub trait Map {
    type Key;
    type Value;

    fn new() -> Self;
    fn find(&self, &Self::Key) -> Option<&Self::Value>;
    fn ins(&mut self, Self::Key, Self::Value) -> &mut Self;
    fn del(&mut self, &Self::Key) -> Option<Self::Value>;
}