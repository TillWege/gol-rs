use iced::{executor, Application, Command, Element, Settings, Text};

fn main() {
    let result = Game::run(Settings::default());
    match result
    {
        Ok(_suc) => println!("Success"),
        Err(err) => println!("Error {}", err),
    };
}

struct Game;

impl Application for Game {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Game, Command<Self::Message>) {
        (Game, Command::none())
    }

    fn title(&self) -> String {
        String::from("Game of Life in Rust")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello, world!").into()
    }
}