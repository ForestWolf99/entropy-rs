use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use tauri::State;

#[derive(Default)]
pub struct Counter(Arc<Mutex<i32>>);

#[derive(Default)]
pub struct CharacterLists {
    printable: Arc<Mutex<Vec<char>>>,
    extended: Arc<Mutex<Vec<char>>>,
}

struct BannedCharacterLists {
    escape: Vec<char>,
    unused: Vec<char>,
}

#[tauri::command]
pub fn setup(character_lists: State<'_, CharacterLists>) {
    let mut escape: Vec<char> = Vec::new();
    escape.push('\t');
    escape.push('\n');
    escape.push('\r');
    escape.push('\'');
    escape.push('\"');
    escape.push('\\');

    let mut unused: Vec<char> = Vec::new();
    unused.push(129 as char);
    unused.push(141 as char);
    unused.push(143 as char);
    unused.push(144 as char);
    unused.push(157 as char);
    unused.push(160 as char);
    unused.push(173 as char);

    let banned_list: BannedCharacterLists = BannedCharacterLists { escape, unused };

    for char in generate_printable() {
        character_lists.printable.lock().unwrap().push(char);
    }

    for char in generate_extended(banned_list) {
        character_lists.extended.lock().unwrap().push(char);
    }
}

fn generate_printable() -> Vec<char> {
    let mut print = Vec::new();

    for i in 33..123 {
        print.push(i as u8 as char);
    }

    print
}

fn generate_extended(banned_character_lists: BannedCharacterLists) -> Vec<char> {
    let mut extended = Vec::new();

    let escape = banned_character_lists.escape.iter().cloned().collect::<Vec<char>>();
    let unused = banned_character_lists.unused.iter().cloned().collect::<Vec<char>>();

    for i in 123..256 {
        let char = i as u8 as char;
        if !escape.contains(&char) && !unused.contains(&char) {
            extended.push(char);
        }
    }

    extended
}

fn calc_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

fn generate_seed() -> u64 {
    let mut seed: Vec<u64> = Vec::new();

    let mut rng = StdRng::seed_from_u64(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );

    for _i in 0..4 {
        let mut rng2 = StdRng::seed_from_u64(rng.next_u64());
        seed = vec![];
        for _j in 0..rng2.gen_range(10, 19) as usize {
            let num1 = rng2.next_u64();
            let mut rng3 = StdRng::seed_from_u64(num1);
            let num2 = rng3.next_u64();
            seed.push(num2);
        }
    }

    let bruh = calc_hash(&seed);
    bruh
}

#[tauri::command]
pub fn debug(character_lists: State<'_, CharacterLists>) -> String {
    let mut charset = character_lists.printable.lock().unwrap().iter().cloned().collect::<Vec<char>>();
    let charset2 = character_lists.extended.lock().unwrap().iter().cloned().collect::<Vec<char>>();

    charset.extend(&charset2);
    charset.iter().collect::<String>()
}

#[tauri::command]
pub fn generate_output(use_extended: bool, use_time: bool, seed: &str, length: i32, character_lists: State<'_, CharacterLists>, ) -> String {
    let mut charset = character_lists.printable.lock().unwrap().iter().cloned().collect::<Vec<char>>();
    log::info!("Generating output");

    if use_extended {
        charset.extend(character_lists.extended.lock().unwrap().iter().cloned());
    }
    let seeed: u64;
    if use_time {
        seeed = generate_seed();
    } else {
        seeed = calc_hash(&seed);
    }

    let mut rng = StdRng::seed_from_u64(seeed);

    let mut result: Vec<char> = Vec::new();

    for _i in 0..length {
        result.push(charset[rng.gen_range(0, charset.len())]);
    }

    let output = result.iter().collect::<String>();
    output
}

#[tauri::command]
pub fn hello_world() -> String {
    "hello world".to_string()
}

#[tauri::command]
pub fn add_count(num: i32, counter: State<'_, Counter>) -> String {
    let mut val = counter.0.lock().unwrap();
    *val += num;
    val.to_string()
}
