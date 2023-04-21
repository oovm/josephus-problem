use josephus::RandomKill;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test() {
    let mut game = RandomKill::new(10, 0.5).into_iter();
    for (count, victim) in game {
        println!("victim-{}: {}", count + 1, victim);
    }
    println!("survivor: {}", game.survivor());
}