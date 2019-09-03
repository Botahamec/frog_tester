extern crate rand;
use std::convert::TryInto;
use rand::Rng;

#[derive(Clone)]
struct Frog {
	id: u32,
	points: i8,
	name: String,
	ability: String
}
impl PartialEq for Frog {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

struct FrogBuilder {frogs: u32}
impl FrogBuilder {
	fn new(&mut self, points: i8, name: &str, ability: &str) -> Frog {
		self.frogs += 1;
		Frog{id: self.frogs, points: points, name: String::from(name), ability: String::from(ability)}
	}
}

/* Missing Frogs:
 * Kermit
 * Frog Phone
 * Loner
 * Peeper
 * Captain Frog
 * Nurse Frog         */
fn initialize_frogs() -> Vec<Frog> {
	let mut fb = FrogBuilder{frogs: 0};
	vec![
		fb.new(5, "Trash", "Discard this card. Nothing else happens."),
		fb.new(5, "Tad", "Draw two cards"),
		fb.new(5, "Dat Boi", "Draw one card."),
		fb.new(5, "Meme Tad", "Discard another frog you control, then pull a card from the discard pile into your hand"),
		fb.new(5, "Definitely a Frog", "Destroy one frog from each opponent."),
		fb.new(5, "Sir Froggo", "Discard this card and one of your opponent's cards."),
		fb.new(5, "Booming Toad", "Draw frogs equal to the number of players, then give them to each player as you see fit."),
		fb.new(5, "Devil Frog", "Destroy another player's frog, or play at any time to destory a frog as it's drawn."),
		fb.new(5, "Great Hopper", "Flip this card. If it lands on the front, destroy another player's frog. Else, draw a card."),
		fb.new(5, "Green Googly Goblin", "Force an opponent to discard their entire hand."),
		fb.new(5, "Hypnotoad", "Flip this card. If it lands on the front, take an opposing frog, otherwise, give that player Hypnotoad."),
		fb.new(5, "Time Frog", "Take another turn."),
		fb.new(5, "Frolord", "Draw cards until you either gain a total of 20 points, reach the maximum number of cards, or empty the deck"),
		fb.new(5, "Pepe", "If you draw this card, discard your entire hand.")
	]
}

fn shuffle(frog_list: Vec<Frog>) -> Vec<Frog> {
	let mut deck : Vec<Frog> = vec![];
	let mut frog_num : u32 = 1;
	while frog_num < frog_list.len().try_into().unwrap() {
		let rn = rand::thread_rng().gen_range(0, frog_num);
		if !deck.contains(&frog_list.clone()[rn as usize]) {
			deck.push(frog_list[rn as usize].clone());
			frog_num += 1;
		}
	}
	deck
}

fn draw_card(deck: &mut Vec<Frog>, hand: &mut Vec<Frog>, discard: &mut Vec<Frog>) -> Frog {
	let frog = deck.pop().unwrap();
	if frog.id == 15 {
		discard.append(hand);
		hand.clear();
	}
	hand.push(frog.clone());
	Ok(frog)
}

fn min_frog(frogs: Vec<Frog>) -> usize {
	let mut i = 0;
    for j in 0..frogs.len() {
        if frogs[j].id < frogs[i].id {
            i = j;
        }
    }
    i
}

fn max_frog(frogs: Vec<Frog>) -> usize {
	let mut i = 0;
    for j in 0..frogs.len() {
        if frogs[j].id > frogs[i].id {
            i = j;
        }
    }
    i
}

fn max_all_frogs(players: Vec<Vec<Frog>>) -> (usize, usize) {
	let mut i = 0;
	let mut max_i = 0;
	for j in 0..players.len() {
		let max_j = max_frog(players[j].clone());
		if players[j][max_j].id > players[i][max_i].id {
			i = j;
			max_i = max_j;
		}
	}
	(i, max_i)
}

fn player_points(player: Vec<Frog>) -> i8 {
	let mut points = 0;
	for frog in player {
		points += frog.points;
	}
	points
}

fn max_player(players: Vec<Vec<Frog>>) -> usize {
	let mut i = 0;
	let mut p = 0;
	for j in 0..players.len() {
		let points = player_points(players[j].clone());
		if points > p {
			i = j;
			p = points;
		}
	}
	i
}

fn min_player(players: Vec<Vec<Frog>>) -> usize {
	let mut i = 0;
	let mut p = 0;
	for j in 0..players.len() {
		let points = player_points(players[j].clone());
		if points < p {
			i = j;
			p = points;
		}
	}
	i
}

fn playable(frog: Frog, deck: Vec<Frog>, discard: Vec<Frog>, player_num: usize, players: Vec<Vec<Frog>>) -> bool {
	match frog.id {
		01 => true,
		02 => deck.len() >= 2,
		03 => deck.len() >= 1,
		04 => players[player_num].len() >= 1 && discard.len() >= 1,
		05 => {
			for i in 0..players.len() {
				if players[i].len() < 1 && i != player_num {return false;}
			}
			true
		},
		06 => {
			for i in 0..players.len() {
				if players[i].len() < 1 && i != player_num {return true;}
			}
			false
		},
		07 => deck.len() >= players.len(),
		08 => {
			for i in 0..players.len() {
				if players[i].len() < 1 && i != player_num {return true;}
			}
			false
		},
		09 => {
			let mut enough_cards = false;
			for i in 0..players.len() {
				if players[i].len() >= 1 && i != player_num {
					enough_cards = true;
					break;
				}
			}
			if deck.len() >= 1 && enough_cards {true}
			else {false}
		},
		10 => {
			for i in 0..players.len() {
				if players[i].len() < 1 && i != player_num {return true;}
			}
			false
		},
		11 => true,
		12 => true,
		13 => true,
		14 => true,
		_ => false
	}
}

fn play_card(mut deck: &mut Vec<Frog>, mut discard: &mut Vec<Frog>, player_num: usize, mut players: &mut Vec<Vec<Frog>>) {

	// filter out unplayable cards
	let mut playable_cards : Vec<usize> = Vec::new();
	for i in 0..players[player_num].len() {
		if playable(players[player_num][i].clone(), deck.to_vec(), discard.to_vec(), player_num, players.to_vec()) {
			playable_cards.push(i);
		}
	}
	if playable_cards.len() == 0 {return;}

	let card_num = playable_cards[rand::thread_rng().gen_range(0, playable_cards.len())];
	let frog = players[player_num].remove(card_num);
	discard.push(frog.clone());
	match frog.id as usize {
		02 => {
			draw_card(&mut deck, &mut players[player_num], &mut discard);
			draw_card(&mut deck, &mut players[player_num], &mut discard);
		},
		03 => {draw_card(&mut deck, &mut players[player_num], &mut discard);},
		04 => {
			let hand = &players.clone()[player_num];
			discard.push(players[player_num].remove(min_frog(hand.to_vec())));
			players[player_num].push(discard.remove(max_frog(discard.to_vec())));
		},
		05 => {
			for i in 0..players.len() {
				let hand = players[i].clone();
				discard.push(players[i].remove(max_frog(hand)));
			}
		},
		06 => {
			let tuple = max_all_frogs(players.clone());
			discard.push(players[tuple.0].remove(tuple.1));
		},
		07 => {
			let mut drawn_cards : Vec<Frog> = Vec::new();
			for _i in 0..players.len() {drawn_cards.push(deck.pop().unwrap());}
			for i in 0..players.len() {
				if i == player_num {players[i].push(drawn_cards.remove(max_frog(drawn_cards.clone())));}
				else {players[i].push(drawn_cards.remove(min_frog(drawn_cards.clone())));}
			}
		},
		08 => {
			let tuple = max_all_frogs(players.clone());
			discard.push(players[tuple.0].remove(tuple.1));
		},
		09 => {
			let coin = rand::thread_rng().gen_range(0, 2);
			if coin == 0 {draw_card(deck, &mut players[player_num], discard);}
			else {
				let tuple = max_all_frogs(players.clone());
				discard.push(players[tuple.0].remove(tuple.1));
			}
		},
		10 => {
			let player = max_player(players.to_vec());
			discard.append(&mut players[player]);
			players[player].clear();
		},
		11 => {
			let coin = rand::thread_rng().gen_range(0, 2);
			if coin == 0 {draw_card(deck, &mut players[player_num], discard);}
			else {
				let player = min_player(players.to_vec());
				players[player].push(frog);
			}
		},
		12 => {
			draw_card(&mut deck, &mut players[player_num], &mut discard);
			play_card(&mut deck, &mut discard, player_num, &mut players);
		},
		13 => {
			let mut points_gained: i8 = 0;
			while points_gained < 20 && deck.len() > 0 {
				let drawn_frog = draw_card(deck, &mut players[player_num], &mut discard);
				points_gained += drawn_frog.points;
			}
		}
		_ => (), // The default is to do nothing
	}
}

fn start_game(frog_list: Vec<Frog>) {
	let mut deck = shuffle(frog_list);
	let mut discard: Vec<Frog> = Vec::new();
	let mut players : Vec<Vec<Frog>> = vec![vec![], vec![], vec![]];
	for i in 0..players.len() {for _j in 0..3 {players[i].push(deck.pop().unwrap());}}
	while deck.len() > 0 {
		for i in 0..3 {
			if deck.len() >= 1 {draw_card(&mut deck, &mut players[i], &mut discard);}
			play_card(&mut deck, &mut discard, i, &mut players);
		}
	}
}

fn main() {
	let frog_list = initialize_frogs();
	start_game(frog_list);
}