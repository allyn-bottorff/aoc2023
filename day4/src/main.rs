#[derive(Debug, PartialEq, Clone)]
struct Card {
    num: u32,
    winners: Vec<u32>,
    drawn: Vec<u32>,
}
impl Card {
    fn from_string(line: &str) -> Self {
        let (prefix, drawn_string) = line.split_once("|").unwrap();
        let drawn: Vec<u32> = drawn_string
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let (card_string, winner_string) = prefix.split_once(":").unwrap();
        let winners: Vec<u32> = winner_string
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let card_vec: Vec<&str> = card_string.split_whitespace().collect();

        Card {
            num: card_vec.last().unwrap().parse::<u32>().unwrap(),
            winners,
            drawn,
        }
    }

    fn score(&self) -> u32 {
        let mut score = 0;
        let mut win = false;
        for num in &self.drawn {
            if self.winners.contains(&num) {
                if win == false {
                    score = 1;
                    win = true;
                } else {
                    score *= 2;
                }
            }
        }
        score
    }

    fn won_cards(&self) -> Option<Vec<usize>> {
        let mut card_ref: usize = 1;
        let mut winners: Vec<usize> = Vec::new();
        for num in &self.drawn {
            if self.winners.contains(&num) {
                winners.push(card_ref + usize::try_from(self.num).unwrap());
                card_ref += 1;
            }
        }
        if card_ref == 1 {
            return None;
        } else {
            return Some(winners);
        }
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let lines = content.lines();

    let mut sum = 0;
    for line in lines {
        let card = Card::from_string(line);
        sum += card.score();
    }
    println!("Part 1 sum: {}", sum);

    let content = std::fs::read_to_string("input.txt").unwrap();
    let lines = content.lines();
    let mut stack: Vec<Vec<&Card>> = Vec::new();
    stack.push(Vec::new());
    let originals: Vec<Card> = lines.map(|l| Card::from_string(l)).collect();

    for j in 0..originals.len() {
        match originals[j].won_cards() {
            Some(w) => {
                for c in w {
                    stack[0].push(&originals[c - 1])
                }
            }
            None => continue,
        }
    }

    let mut i: usize = 0;
    loop {
        for j in 0..stack[i].len() {
            match stack[i][j].won_cards() {
                Some(w) => {
                    if i == stack.len() - 1 {
                        stack.push(Vec::new());
                    }
                    for c in w {
                        let card = &originals[c - 1];
                        stack[i + 1].push(card);
                    }
                }
                None => continue,
            }
        }
        if i == stack.len() - 1 {
            break;
        }
        i += 1;
    }

    let mut sum = 0;

    for deck in stack {
        sum += deck.len();
    }
    sum += originals.len();

    println!("Pt. 2 sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_from_string() {
        let card_string = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::from_string(card_string);

        let expected_card = Card {
            num: 1,
            winners: vec![41, 48, 83, 86, 17],
            drawn: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        assert_eq!(card, expected_card)
    }

    #[test]
    fn test_card1_score() {
        let card_string = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::from_string(card_string);
        let score = card.score();

        assert_eq!(score, 8)
    }
}
