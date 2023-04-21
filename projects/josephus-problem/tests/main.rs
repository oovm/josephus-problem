use crossbeam_skiplist::map::Entry;
use crossbeam_skiplist::SkipMap;
use josephus::RandomKill;

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
    let last = unsafe {
        game.into_iter().last().unwrap_unchecked()
    };
    println!("最后生还者为: {:?}", last);
}


#[test]
fn test_last() {
    let mut count = SkipMap::<usize, usize>::default();
    for i in 0..=10000 {
        let last = unsafe {
            RandomKill::new(10, 0.5).into_iter().last().unwrap_unchecked()
        };
        let new = match count.get(&last.id) {
            Some(v) => {
                v.value() + 1
            }
            None => {
                1
            }
        };
        count.insert(last.id, new);
    }
    for k in count.iter() {
        println!("{}: {}", k.key(), k.value()   );
    }
}