use std::io;

enum SpaceType {
    Character(String),
    Hidden,
}

struct Space {
    space_type: SpaceType,
    character: char,
}

struct StickMan {
    left_hand: bool,
    right_hand: bool,
    left_foot: bool,
    right_foot: bool,
    torso: bool,
}

impl Space {
    fn new(character: char) -> Space {
        Space {
            space_type: SpaceType::Hidden,
            character,
        }
    }

    fn reveal(&mut self) {
        self.space_type = SpaceType::Character(self.character.to_string());
    }
}

struct GameManger {
    spaces: Vec<Space>,
    stick: StickMan,
    word: String,
    attempts: i8,
}

impl GameManger {
    fn new(spaces: Vec<Space>, word: String) -> GameManger {
        GameManger {
            spaces,
            stick: StickMan {
                left_hand: false,
                right_hand: false,
                left_foot: false,
                right_foot: false,
                torso: false,
            },
            word,
            attempts: 4,
        }
    }

    fn guess(&mut self, guess: char) {
        let mut correct = false;

        for space in &mut self.spaces {
            if space.character == guess {
                space.reveal();
                correct = true;
            }
        }

        if !correct {
            self.stick.left_hand = true;
        }
    }

    fn is_game_over(&self) -> bool {
        let mut game_over = true;

        for space in &self.spaces {
            if let SpaceType::Hidden = space.space_type {
                game_over = false;
            }
        }

        game_over
    }

    fn print_spaces(&self) {
        for space in &self.spaces {
            match &space.space_type {
                SpaceType::Character(character) => print!("{} ", character),
                SpaceType::Hidden => print!("_ "),
            }
        }
    }

    fn print_stick(&self) {
        if self.stick.left_hand {
            println!("  O");
        } else {
            println!(" ");
        }

        if self.stick.torso {
            println!("  |");
        } else {
            println!(" ");
        }

        if self.stick.left_foot {
            print!(" /");
        } else {
            print!(" ");
        }

        if self.stick.right_foot {
            println!(" \\");
        } else {
            println!(" ");
        }
    }
}

fn main() {
    println!("Input a word for the other player to guess");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let word = input.trim();

    let mut spaces: Vec<Space> = Vec::new();

    for character in word.chars() {
        spaces.push(Space::new(character));
    }

    let mut game_manager = GameManger::new(spaces, word.to_string());

    while game_manager.attempts > 0 {
        game_manager.print_spaces();
        println!();
        game_manager.print_stick();
        println!();

        println!("Input a letter to guess");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let guess = input.trim().chars().next().unwrap();

        game_manager.guess(guess);

        if game_manager.is_game_over() {
            println!("You win!");
            break;
        }

        game_manager.attempts -= 1;
    }
}
