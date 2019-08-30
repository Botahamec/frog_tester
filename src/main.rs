struct Frog {
	id: u32,
	points: i8,
	name: String,
	ability: String
}

struct FrogBuilder {frogs: u32}
impl FrogBuilder {
	fn new(&self, points: i8, name: &str, ability: &str) -> Frog {
		self.frogs += 1;
		Frog{id: self.frogs, points: points, name: String::from(name), ability: String::from(ability)}
	}
}

fn initialize_frogs() -> Vec<Frog> {
	let fb = FrogBuilder{frogs: 0};
	vec![
		fb.new(1, "Trash", "Discard this card. Nothing else happens."),
		fb.new(-3, "Loner", "This card subtracts three points frim your hand as long as you have it."),
		fb.new(2, "Tad", "Draw two cards"),
		fb.new(2, "Dat Boi", "Draw one card."),
		fb.new(2, "Pepper", "Choose a frog. Its owner may not use that frog until their next turn."),
		fb.new(3, "Definitely a Frog", "Destroy one frog from each opponent."),
		fb.new(3, "Kermit", "Shuffle the discard pile into the deck."),
		fb.new(3, "Nurse Frog", "You may take any frog of four points or less."),
		fb.new(3, "Sir Froggo", "Discard this card and one of your opponent's cards."),
		fb.new(3, "Meme Tad", "Discard another frog you control, return a frog from the discard pile to your hand.")
	]
}

fn main() {
	let frogs = initialize_frogs();
}
