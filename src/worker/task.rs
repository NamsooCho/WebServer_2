pub trait Task: Send {
    fn execute(&mut self);
}
