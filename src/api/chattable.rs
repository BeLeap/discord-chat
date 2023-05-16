pub trait Chattable {
    fn chat(self: &Self, instruction: String) -> String;
}
