pub trait Item {
    fn name(&self) -> &str;
    fn definition(&self) -> &str;
    fn code(&self) -> &str;
}
