#![feature(vec_into_raw_parts)]

mod assets;
mod game;
mod word_bank;

use game::Game;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Copy, Clone, PartialEq)]
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

    pub fn as_str(&self) -> &'static str {
        use CharStatus::*;

        match *self {
            Excluded => "excluded",
            Shift => "shift",
            Exact => "exact",
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

#[derive(Clone)]
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
    ToggleChar(usize),
    CommitWord,
    Reset,
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
        use CharStatus::*;

        match msg {
            SelectWord(word) => {
                self.current_guess = Guess::from_string(&word);
                for (pos, cha) in self.current_guess.characters.iter_mut().enumerate() {
                    if self.game.has_exact(pos, cha.char) {
                        cha.status = CharStatus::Exact;
                    }
                }
                true
            },
            ToggleChar(pos) => {
                self.current_guess.characters[pos].status.toggle();
                true
            },
            CommitWord => {
                for (pos, charstatus) in self.current_guess.characters.iter().enumerate() {
                    match charstatus.status {
                        Excluded => {
                            self.game.add_exclussion(charstatus.char);
                        },
                        Shift => {
                            self.game.char_shift(pos, charstatus.char);
                        },
                        Exact => {
                            self.game.char_exact(pos, charstatus.char);
                        }
                    }
                }
                self.previous_guesses.push(self.current_guess.clone());
                let next_word = self.game.words().iter().nth(0).map(|w| *w).unwrap_or(".....");
                self.current_guess = Guess::from_string(next_word);
                for (pos, cha) in self.current_guess.characters.iter_mut().enumerate() {
                    if self.game.has_exact(pos, cha.char) {
                        cha.status = CharStatus::Exact;
                    }
                }
                true
            },
            Reset => {
                self.game.reset();
                self.previous_guesses.clear();
                self.current_guess = Guess::from_string(self.game.best_guess());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let on_word_change = link.callback(|e: Event| {
            let select = e.target().unwrap().unchecked_into::<HtmlSelectElement>();
            GameMessage::SelectWord(select.value())
        });

        html! {
            <div class="container">
                <h3>{ format!("Words Available: {}", self.game.words().len()) }</h3>
                <div class="previous_guesses">
                {
                    self.previous_guesses.iter().map(|guess| {
                        html!{
                            <div class="guess">
                            {
                                guess.characters.iter().map(|gc|{
                                    html!{
                                        <button class={gc.status.as_str()}>{ gc.char }</button>
                                    }
                                }).collect::<Html>()
                            }
                                <br />
                            </div>
                        }
                    }).collect::<Html>()
                }
                </div>
                <div class="current_guess guess">
                {
                    self.current_guess.characters.into_iter().enumerate().map(|(pos, gc)|{
                        html!{
                            <button
                                class={gc.status.as_str()}
                                onclick={link.callback(move |_| GameMessage::ToggleChar(pos))}
                            >{ gc.char }</button> }
                    }).collect::<Html>()
                }
                </div>
                <hr />
                <div class="word_selection">
                    <select onchange={on_word_change}>
                    {
                        self.game.words().into_iter().take(200).map(|word|{
                            html!{ <option selected={self.current_guess.is_for(word)} value={word.to_owned()}>{ word }</option> }
                        }).collect::<Html>()
                    }
                    </select>
                    <br />
                    <button onclick={link.callback(|_| GameMessage::CommitWord)}>
                        { "Commit Guess" }
                    </button>
                    <button onclick={link.callback(|_| GameMessage::Reset)}>
                        { "Start Over" }
                    </button>
                </div>
            </div>
        }
    }
}

fn main() {
    let window = web_sys::window().expect("a window");
    let document = window.document().expect("a document");
    let element = document.get_element_by_id("wordle-server").unwrap();
    yew::start_app_in_element::<GameComponent>(element);
}
