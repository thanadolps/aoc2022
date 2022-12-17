/// The different hand signs that can be played in a game of rock-paper-scissors
///
/// The `HandSign` enum is designed so that its values correspond to player scores when converted to `usize`.
#[derive(Clone, Copy, Debug)]
enum HandSign {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

/// The possible outcomes of a battle.
///
/// The `BattleResult` enum is designed so that its values correspond to player scores when converted to `usize`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum BattleResult {
    Lost = 0,
    Draw = 3,
    Won = 6,
}

/// Calculates a score based on two hand signs.
fn calculate_score(my_sign: HandSign, opponent_sign: HandSign) -> usize {
    // Bring the variants of the `BattleResult` and `HandSign` enums into scope.
    use BattleResult::*;
    use HandSign::*;

    // Convert the value of `my_sign` to an `usize`.
    let shape_score = my_sign as usize;

    // Determine the result of a battle based on the hand signs.
    let battle_result = match (my_sign, opponent_sign) {
        // If `my_sign` loses to `opponent_sign`, the result is Lost.
        (Rock, Paper) | (Paper, Scissor) | (Scissor, Rock) => Lost,
        // If `my_sign` ties with `opponent_sign`, the result is Draw.
        (Rock, Rock) | (Paper, Paper) | (Scissor, Scissor) => Draw,
        // If `my_sign` wins against `opponent_sign`, the result is Won.
        (Rock, Scissor) | (Paper, Rock) | (Scissor, Paper) => Won,
    };
    // Convert the value of `battle_result` to an `usize`.
    let battle_score = battle_result as usize;

    // Return the sum of `shape_score` and `battle_score`.
    shape_score + battle_score
}

pub fn part_1(input: &str) -> usize {
    // Split the input into lines and map over them.
    input
        .lines()
        .map(|line| {
            // Convert the line into an array of three bytes.
            let [opponent_symbol, _, my_symbol]: [u8; 3] =
                line.trim().as_bytes().try_into().unwrap();

            // The first byte indicates the opponent's hand sign.
            use HandSign::*;
            let opponent_sign = match opponent_symbol {
                b'A' => Rock,
                b'B' => Paper,
                b'C' => Scissor,
                _ => unreachable!(),
            };

            // The third byte indicates the player's hand sign.
            let my_sign = match my_symbol {
                b'X' => Rock,
                b'Y' => Paper,
                b'Z' => Scissor,
                _ => unreachable!(),
            };

            // Calculate the score for the current line.
            calculate_score(my_sign, opponent_sign)
        })
        // Sum the scores for all lines.
        .sum()
}

pub fn part_2(input: &str) -> usize {
    // Split the input into lines and map over them.
    input
        .lines()
        .map(|line| {
            // Convert the line into an array of three bytes.
            let [opponent_symbol, _, result_symbol]: [u8; 3] =
                line.trim().as_bytes().try_into().unwrap();

            // The first byte indicates the opponent's hand sign.
            use HandSign::*;
            let opponent_sign = match opponent_symbol {
                b'A' => Rock,
                b'B' => Paper,
                b'C' => Scissor,
                _ => unreachable!(),
            };

            // The third byte indicates the game result.
            use BattleResult::*;
            let battle_result = match result_symbol {
                b'X' => Lost,
                b'Y' => Draw,
                b'Z' => Won,
                _ => unreachable!(),
            };

            // Determine the player's hand sign based on the opponent's sign and the game result.
            let my_sign = match (opponent_sign, battle_result) {
                (Paper, Lost) | (Rock, Draw) | (Scissor, Won) => Rock,
                (Scissor, Lost) | (Paper, Draw) | (Rock, Won) => Paper,
                (Rock, Lost) | (Scissor, Draw) | (Paper, Won) => Scissor,
            };

            // Convert the player's hand sign and the game result to scores.
            let shape_score = my_sign as usize;
            let battle_score = battle_result as usize;

            // Return the sum of the scores.
            shape_score + battle_score
        })
        // Sum the scores for all lines.
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(r#"
A Y
B X
C Z"#.trim() => 15; "part1")]
    fn part1(input: &str) -> usize {
        part_1(input)
    }

    #[test_case(r#"
    A Y
    B X
    C Z"#.trim() => 12; "part2")]
    fn part2(input: &str) -> usize {
        part_2(input)
    }
}
