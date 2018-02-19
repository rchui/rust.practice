#[derive(Debug)]
struct Message {
    text: String,
    name: String
}

trait Concatable {
    fn concat(&self) -> String;
}

impl Concatable for Message {
    fn concat(&self) -> String {
        let mut output = String::from("");
        output.push_str(&self.text);
        output.push_str(&self.name);
        output
    }
}

fn main() {
    let message = Message{text: String::from("Hello World "), name: String::from("Ryan")};
    println!("{}", message.concat());
}
