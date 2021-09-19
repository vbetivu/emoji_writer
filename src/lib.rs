mod alphabet;

use std::env;

use crate::alphabet as Alphabet;

const SPACE_ARG: &str = "-sp";
const STROKE_ARG: &str = "-st";

pub struct Config {
    space_emoji: String,
    stroke_emoji: String,
    input: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // skip binary path
        args.next();

        let mut space_emoji: Option<String> = Option::None;
        let mut stroke_emoji: Option<String> = Option::None;
        let mut input: Vec<String> = Vec::new();

        loop {
            if let Some(arg) = args.next() {
                if arg == SPACE_ARG {
                    space_emoji = args.next();

                    validate_emoji(&space_emoji)?;
                } else if arg == STROKE_ARG {
                    stroke_emoji = args.next();

                    validate_emoji(&space_emoji)?;
                } else {
                    input.push(arg);
                }
            } else if space_emoji.is_none() || stroke_emoji.is_none() || input.len() == 0 {
                return Result::Err("Not enough arguments!");
            } else {
                break;
            }
        }

        Result::Ok(Config {
            space_emoji: space_emoji.unwrap(),
            stroke_emoji: stroke_emoji.unwrap(),
            input: input.join(" "),
        })
    }
}

fn validate_emoji(emoji: &Option<String>) -> Result<(), &'static str> {
    if let Some(emoji_string) = emoji {
        if !emoji_string.starts_with(":") || !emoji_string.ends_with(":") {
            return Result::Err("Emoji should start with ':' and end with ':'!");
        }
    } else {
        return Result::Err("Emoji was not provided!");
    }

    return Result::Ok(());
}

struct Text {
    config: Config,
    output: Vec<String>,
}

impl Text {
    fn new(config: Config) -> Text {
        Text {
            config: config,
            output: Vec::new(),
        }
    }

    fn generate(&mut self) -> Result<(), &'static str> {
        let text_chars: Vec<char> = self.config.input.chars().collect();
        let mut row_count: Option<usize> = Option::None;

        for row_number in 0..5 {
            let mut row_text = String::new();

            row_text += &self.config.space_emoji;

            for character in &text_chars {
                match character.to_uppercase().next().unwrap() {
                    'A' => self.draw_character(&mut row_text, Alphabet::A.get(row_number).unwrap()),
                    'B' => self.draw_character(&mut row_text, Alphabet::B.get(row_number).unwrap()),
                    'C' => self.draw_character(&mut row_text, Alphabet::C.get(row_number).unwrap()),
                    'D' => self.draw_character(&mut row_text, Alphabet::D.get(row_number).unwrap()),
                    'E' => self.draw_character(&mut row_text, Alphabet::E.get(row_number).unwrap()),
                    'F' => self.draw_character(&mut row_text, Alphabet::F.get(row_number).unwrap()),
                    'G' => self.draw_character(&mut row_text, Alphabet::G.get(row_number).unwrap()),
                    'H' => self.draw_character(&mut row_text, Alphabet::H.get(row_number).unwrap()),
                    'I' => self.draw_character(&mut row_text, Alphabet::I.get(row_number).unwrap()),
                    'J' => self.draw_character(&mut row_text, Alphabet::J.get(row_number).unwrap()),
                    'K' => self.draw_character(&mut row_text, Alphabet::K.get(row_number).unwrap()),
                    'L' => self.draw_character(&mut row_text, Alphabet::L.get(row_number).unwrap()),
                    'M' => self.draw_character(&mut row_text, Alphabet::M.get(row_number).unwrap()),
                    'N' => self.draw_character(&mut row_text, Alphabet::N.get(row_number).unwrap()),
                    'O' => self.draw_character(&mut row_text, Alphabet::O.get(row_number).unwrap()),
                    'P' => self.draw_character(&mut row_text, Alphabet::P.get(row_number).unwrap()),
                    'Q' => self.draw_character(&mut row_text, Alphabet::Q.get(row_number).unwrap()),
                    'R' => self.draw_character(&mut row_text, Alphabet::R.get(row_number).unwrap()),
                    'S' => self.draw_character(&mut row_text, Alphabet::S.get(row_number).unwrap()),
                    'T' => self.draw_character(&mut row_text, Alphabet::T.get(row_number).unwrap()),
                    'U' => self.draw_character(&mut row_text, Alphabet::U.get(row_number).unwrap()),
                    'V' => self.draw_character(&mut row_text, Alphabet::V.get(row_number).unwrap()),
                    'W' => self.draw_character(&mut row_text, Alphabet::W.get(row_number).unwrap()),
                    'X' => self.draw_character(&mut row_text, Alphabet::X.get(row_number).unwrap()),
                    'Y' => self.draw_character(&mut row_text, Alphabet::Y.get(row_number).unwrap()),
                    'Z' => self.draw_character(&mut row_text, Alphabet::Z.get(row_number).unwrap()),
                    ' ' => {
                        self.draw_character(&mut row_text, Alphabet::SPACE.get(row_number).unwrap())
                    }
                    _ => {
                        return Result::Err("Invalid character! Use english letter only!");
                    }
                }
                row_text += &self.config.space_emoji;
            }

            if row_count.is_none() {
                row_count = Option::Some(row_text.split("::").collect::<Vec<&str>>().len());

                self.output
                    .push(self.config.space_emoji.repeat(row_count.unwrap()))
            }

            self.output.push(row_text);
        }

        if row_count.is_some() {
            self.output
                .push(self.config.space_emoji.repeat(row_count.unwrap()));
        }

        return Result::Ok(());
    }

    fn print(&self) {
        for row in &self.output {
            println!("{}", row);
        }
    }

    fn draw_character(&mut self, row: &mut String, pattern: &[bool]) {
        for is_stroke in pattern {
            if *is_stroke {
                row.push_str(&self.config.stroke_emoji);
            } else {
                row.push_str(&self.config.space_emoji);
            }
        }
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let mut text = Text::new(config);

    text.generate()?;

    text.print();

    return Result::Ok(());
}
