use josephus::RandomKill;
use std::collections::BTreeMap;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let game = RandomKill::new(100, 0.5);
    for (i, man) in RandomKill::new(100, 0.5).into_iter().enumerate() {
        println!("受害者 {} 号: {} (存活 {} 轮)", i + 1, man.id + 1, man.steps);
    }
    let last = unsafe { game.into_iter().last().unwrap_unchecked() };
    println!("最后生还者为: {:?}", last);
}

#[test]
fn test_last() {
    let mut count = BTreeMap::default();
    let total = 10000000;
    for _ in 0..total {
        // SAFETY: definitely has survivor
        unsafe {
            let last = RandomKill::new(10, 0.5).into_iter().last().unwrap_unchecked();
            *count.entry(last.id).or_insert(0) += 1usize;
        }
    }
    for (k, v) in count.iter() {
        println!("{}: {}", k + 1, *v as f32 / total as f32);
    }
}
