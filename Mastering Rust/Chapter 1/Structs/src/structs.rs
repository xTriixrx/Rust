// structs.rs

struct Player {
    name: String,
    iq: u8,
    friends: u8,
    score: u16
}

fn bump_player_score(player: &mut Player, score: u16)
{
    player.score += score;
}

fn player_stats(player: Player)
{
    println!("Player stats:");
    println!("Name: {}", player.name);
    println!("IQ: {}", player.iq);
    println!("Friends: {}", player.friends);
    println!("Score: {}", player.score);
}

fn main()
{
    let name = "Alice".to_string();
    let mut player = Player{name, iq: 171, friends: 134, score: 1129};

    bump_player_score(&mut player, 100);
    player_stats(player);
}