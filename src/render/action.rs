pub trait Action where Self: Sized {
    fn apply(&self);
}