struct Game {
    id: u32,
    rounds: Vec<Round>,
}

struct Round {
    green: Option<u32>,
    red: Option<u32>,
    blue: Option<u32>,
}

static MAX_COUNT_RED: u32 = 12;
static MAX_COUNT_GREEN: u32 = 13;
static MAX_COUNT_BLUE: u32 = 14;

fn main() {
    let input = include_str!("input.txt");
    let games = input.lines().map(parse_line).collect::<Vec<_>>();

    let valid_games = games.iter().filter(|game| is_valid_game(game)).collect::<Vec<_>>();

    // sum all valid games' ids
    let sum = valid_games.iter().map(|game| game.id).sum::<u32>();
    println!("Sum of ids: {}", sum);
}

fn parse_line(line: &str) -> Game {
    // get the game id from "Game {number}:"
    let id = parse_game_id(line).expect("Failed to parse game id");
    let rounds = parse_rounds(line).expect("Failed to parse rounds");

    Game { id, rounds }
}

fn parse_game_id(line: &str) -> Option<u32> {
    line.strip_prefix("Game ")
        .and_then(|s| s.split(":").next())
        .and_then(|s| s.parse::<u32>().ok())
}

fn parse_rounds(line: &str) -> Option<Vec<Round>> {
    let rounds_substr = line.split(":").nth(1)?;
    let rounds = rounds_substr.split(";").map(parse_round).collect::<Vec<_>>();

    Some(rounds)
}

fn parse_round(round_substr: &str) -> Round {
    let mut round = Round {
        green: None,
        red: None,
        blue: None,
    };

    let colors = round_substr.split(",").collect::<Vec<_>>();

    colors.iter().for_each(|color| {
        if color.contains("green") {
            round.green = parse_color(color);
        } else if color.contains("red") {
            round.red = parse_color(color);
        } else if color.contains("blue") {
            round.blue = parse_color(color);
        }
    });

    round
}

fn parse_color(color_substr: &str) -> Option<u32> {
    color_substr
        .strip_suffix("green")
        .or_else(|| color_substr.strip_suffix("red"))
        .or_else(|| color_substr.strip_suffix("blue"))
        .and_then(|s| Some(s.trim()))
        .and_then(|s| s.parse::<u32>().ok())
}

fn is_valid_round(round: &Round) -> bool {
    let mut valid = true;

    if let Some(green) = round.green {
        valid = valid && green <= MAX_COUNT_GREEN;
    }

    if let Some(red) = round.red {
        valid = valid && red <= MAX_COUNT_RED;
    }

    if let Some(blue) = round.blue {
        valid = valid && blue <= MAX_COUNT_BLUE;
    }

    valid
}

fn is_valid_game(game: &Game) -> bool {
    let mut valid = true;

    game.rounds.iter().for_each(|round| {
        valid = valid && is_valid_round(round);
    });

    valid
}