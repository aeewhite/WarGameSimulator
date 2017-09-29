#![feature(test)]

extern crate cards;
extern crate test;

enum Player{
	Left,
	Right
}

struct WarGame{
	winner: Player,
	turns: u32
}

fn play()->WarGame{
	// Get a new deck
	let mut deck = cards::deck::Deck::new_shuffled();
	
	// Split Deck between players
	let mut left = deck.draw_n(26).unwrap();
	let mut right = deck.draw_n(26).unwrap();

	let mut turns = 0;

	// Game loop
	loop {
		turns += 1;
		// Check if someone won (no more cards)
		if left.len() == 52 {
			return WarGame{winner: Player::Left, turns: turns};
		}
		else if right.len() == 52 {
			return WarGame{winner: Player::Right, turns: turns};
		}

		// Play the "hand"
		let left_card = left.pop().expect("Initial Left Play");
		let right_card = right.pop().expect("Initial Right Play");

		if left_card.value.gt(&right_card.value){
			left.insert(0, left_card);
			left.insert(0, right_card);
		}
		else if right_card.value.gt(&left_card.value){
			right.insert(0, right_card);
			right.insert(0, left_card);
		}
		else {
			let mut pool = Vec::with_capacity(6);
			pool.push(left_card);
			pool.push(right_card);
			loop{
				// Draw 4, compare last (if player runs out, they lose)
				let l;
				let r;
				// From the left
				if left.len() < 4{
					right.append(&mut left); //Give away all cards
					right.append(&mut pool);
					break;
				}
				else{
					pool.push(left.pop().unwrap());
					pool.push(left.pop().unwrap());
					pool.push(left.pop().unwrap());
					l = left.pop().unwrap();
				}

				// From the right
				if right.len() < 4{
					left.append(&mut right); //Give away all cards
					left.append(&mut pool);
					left.insert(0, l); //left has already played
					break;
				}
				else{
					pool.push(right.pop().unwrap());
					pool.push(right.pop().unwrap());
					pool.push(right.pop().unwrap());
					r = right.pop().unwrap();
				}

				// Compare
				if l.value.gt(&r.value){
					left.insert(0, l);
					left.insert(0, r);
					left.append(&mut pool);
					break;
				}
				else if r.value.gt(&l.value){
					right.insert(0, l);
					right.insert(0, r);
					right.append(&mut pool);
					break;
				}
				else{
					//There is another tie, push cards to pool and loop again
					pool.push(l);
					pool.push(r);
				}
			}
		}
	}
}

fn main() {
	let mut games = Vec::new();
	for _ in 0..500_000{
		games.push(play());
	}
	let mut left_wins = 0;
	let mut right_wins = 0;
	let mut total_turns = 0;

	for game in &games{
		total_turns += game.turns;
		match game.winner{
			Player::Left => left_wins += 1,
			Player::Right => right_wins += 1
		}
	}

	println!("Left Wins: {:?} ({:?}%), Right Wins: {:?} ({:?}%)", 
				left_wins, 
				left_wins as f32 / games.len() as f32 * 100f32,
				right_wins,
				right_wins as f32 / games.len() as f32 * 100f32,);
	println!("Average # of Turns: {:?}", total_turns as f32 / games.len() as f32);
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	#[bench]
	fn bench_play_game(b: &mut Bencher) {
		b.iter(|| play());
	}
}
