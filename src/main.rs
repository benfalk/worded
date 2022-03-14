#![feature(vec_into_raw_parts)]

mod game;
mod word_bank;
mod assets;

use game::Game;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

#[derive(Copy, Clone)]
enum CharStatus {
    Excluded,
    Shift,
    Exact,
}

impl CharStatus {
    pub fn toggle(&mut self) {
        use CharStatus::*;

        match *self {
            Excluded => *self = Shift,
            Shift => *self = Exact,
            Exact => *self = Excluded,
        }
    }
}

impl Default for CharStatus {
    fn default() -> Self {
        CharStatus::Excluded
    }
}

#[derive(Copy, Clone)]
struct GuessCharacter {
    char: char,
    status: CharStatus,
}

impl Default for GuessCharacter {
    fn default() -> Self {
        Self {
            char: ' ',
            status: Default::default(),
        }
    }
}

struct Guess {
    characters: [GuessCharacter; 5],
    word: String,
}

impl Guess {
    pub fn from_string(string: &str) -> Self {
        let mut characters = [GuessCharacter::default(); 5];
        for (pos, char) in string.chars().enumerate().take(5) {
            characters[pos].char = char;
        }
        Self {
            characters,
            word: string.to_owned(),
        }
    }

    pub fn is_for(&self, word: &str) -> bool {
        self.word == word
    }
}

enum GameMessage {
    SelectWord(String),
}

struct GameComponent {
    game: Game,
    previous_guesses: Vec<Guess>,
    current_guess: Guess,
}

impl Component for GameComponent {
    type Message = GameMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let game = Game::new();
        let best_guess = game.best_guess();
        let current_guess = Guess::from_string(best_guess);

        Self {
            game,
            previous_guesses: vec![],
            current_guess,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use GameMessage::*;

        match msg {
            SelectWord(word) => {
                self.current_guess = Guess::from_string(&word);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let on_word_change = link.callback(|e: Event|{
            let select = e.target().unwrap().unchecked_into::<HtmlSelectElement>();
            GameMessage::SelectWord(select.value())
        });

        html! {
            <div class="container">
                <h3>{ "Wordle Solver" }</h3>
                <div class="current_guess">
                {
                    self.current_guess.characters.into_iter().enumerate().map(|(_pos, gc)|{
                        html!{ <button>{ gc.char }</button> }
                    }).collect::<Html>()
                }
                </div>
                <div class="word_selection">
                    <select onchange={on_word_change}>
                    {
                        self.game.words().into_iter().map(|word|{
                            html!{ <option selected={self.current_guess.is_for(word)} value={word.to_owned()}>{ word }</option> }
                        }).collect::<Html>()
                    }
                    </select>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<GameComponent>();
}
