use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::str::FromStr;
use std::fmt;

const NEW_LINE: &str = "\r\n";
const DOUBLE_NEW_LINE: &str = "\r\n\r\n";

#[derive(Clone, Hash, PartialEq, Eq)]
struct Player {
    name: String,
    cards: VecDeque<usize>
}

impl FromStr for Player {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut lines = text.split(NEW_LINE);
        let name: String = match lines.next() {
            Some(header) => {
                if header.ends_with(":") {
                    header[0..header.len()-1].to_string()
                } else {
                    return Err("Invalid header format".to_string())
                }
            },
            None => return Err("Header is missing".to_string())
        };
        let mut cards: VecDeque<usize> = VecDeque::new();
        while let Some(s) = lines.next() {
            cards.push_back(match s.parse::<usize>() {
                Ok(c) => c,
                Err(_) => return Err(format!("Invalid card: {}", s))
            });
        }
        Ok(Player {
            name,
            cards
        })
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.name, self.cards)
    }
}

impl Player {
    fn has_no_cards(&self) -> bool {
        self.cards.len() == 0
    }

    fn calculate_score(&self) -> usize {
        let mut score: usize = 0;
        let mut value: usize = self.cards.len();
        for card in self.cards.iter() {
            score += card * value;
            value -= 1;
        }
        score
    }

    fn sub_deck(&self, take_cards: usize) -> Player {
        Player {
            name: self.name.clone(),
            cards: self.cards.iter().take(take_cards).cloned().collect()
        }
    }
}

struct RecursiveCombat {
    round: usize,
    previous_rounds: HashMap<(Player,Player),usize>, // map from (player1,player2) to round
    player1: Player,
    player2: Player
}

impl RecursiveCombat {
    fn new(player1: Player, player2: Player) -> Self {
        RecursiveCombat {
            round: 0,
            previous_rounds: HashMap::new(),
            player1,
            player2
        }
    }

    fn play_round(&mut self) {
        // save previous state
        self.previous_rounds.insert((self.player1.clone(),self.player2.clone()), self.round);
        // start this round
        self.round += 1;
        // deal the cards
        let card1 = self.player1.cards.pop_front().expect("Cannot play if player1 has no cards");
        let card2 = self.player2.cards.pop_front().expect("Cannot play if player2 has no cards");
        // find the winnder
        let winner_name: String;
        if self.player1.cards.len() >= card1 && self.player2.cards.len() >= card2 {
            // recurse
            let mut sub_game = RecursiveCombat::new(self.player1.sub_deck(card1), self.player2.sub_deck(card2));
            loop {
                sub_game.play_round();
                if let Some(sub_winner) = sub_game.has_winner() {
                    println!("{} wins sub-game after {} rounds!", sub_winner.name, sub_game.round);
                    winner_name = sub_winner.name.clone();
                    break;
                }
            }
        } else {
            // like non-recursive combat
            assert_ne!(card1, card2);
            if card1 > card2 {
                winner_name = self.player1.name.clone();
            } else {
                winner_name = self.player2.name.clone();
            }
        }
        // put cards in winners deck
        if winner_name == self.player1.name {
            self.player1.cards.push_back(card1);
            self.player1.cards.push_back(card2);
        } else {
            self.player2.cards.push_back(card2);
            self.player2.cards.push_back(card1);
        }
    }

    fn has_winner(&self) -> Option<&Player> {
        if self.player1.has_no_cards() {
            Some(&self.player2)
        } else if self.player2.has_no_cards() {
            Some(&self.player1)
        } else {
            match self.previous_rounds.get(&(self.player1.clone(),self.player2.clone())) {
                Some(_) => Some(&self.player1),
                None => None
            }
        }
    }
} 

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut players: VecDeque<Player> = text.split(DOUBLE_NEW_LINE).map(|s| s.parse()
            .expect(&format!("Error parsing player: {}", s))).collect();
        let mut game = RecursiveCombat {
            round: 0,
            previous_rounds: HashMap::new(),
            player1: players.pop_front().expect("There should be a Player 1"),
            player2: players.pop_front().expect("There should be a Player 2")
        };
        loop {
            game.play_round();
            if let Some(winner) = game.has_winner() {
                println!("{} wins after {} rounds!", winner.name, game.round);
                println!("{}", winner);
                println!("Score: {}", winner.calculate_score());
                break;
            }
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}