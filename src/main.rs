use lazy_static::lazy_static;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::{
    collections::HashMap,
    io::{self, Write},
};

const MAX_FRET: u8 = 24;

const NOTES: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

lazy_static! {
    static ref EQUIVALENTS: HashMap<&'static str, &'static str> = HashMap::from([
        ("A#", "Bb"),
        ("Bb", "A#"),
        ("C#", "Db"),
        ("Db", "C#"),
        ("D#", "Eb"),
        ("Eb", "D#"),
        ("F#", "Gb"),
        ("Gb", "F#"),
        ("G#", "Ab"),
        ("Ab", "G#"),
    ]);
}

const STRINGS: [&str; 5] = ["B", "E", "A", "D", "G"];

fn note_from_string_and_fret(string: &str, fret: u8) -> Result<&str, io::Error> {
    match NOTES.binary_search(&string) {
        Err(_) => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Note {} is not valid", string),
        )),
        Ok(index) => {
            let mut fret_index = index + fret as usize;
            while fret_index > NOTES.len() - 1 {
                fret_index -= NOTES.len();
            }
            Ok(NOTES[fret_index])
        }
    }
}

fn note_equals(note: &str, test: &str) -> bool {
    if test.len() > 2 {
        return false;
    }
    let mut adjusted = String::with_capacity(2);
    for (i, c) in test.chars().enumerate() {
        if i == 0 {
            adjusted.push_str(c.to_uppercase().to_string().as_str())
        } else {
            adjusted.push(c)
        }
    }
    if note == adjusted {
        return true;
    };
    if let Some(equiv) = EQUIVALENTS.get(note) {
        return *equiv == adjusted;
    };
    false
}

fn random_fret() -> u8 {
    let mut rng = thread_rng();
    rng.gen_range(0..=MAX_FRET)
}

fn random_choice<T>(collection: &[T]) -> Option<&T> {
    let mut rng = thread_rng();
    collection.choose(&mut rng)
}

fn create_fretboard(string: &str, fret: u8) -> String {
    let mut fretboard = String::new();
    for &s in STRINGS.iter().rev() {
        fretboard.push_str(if s == string && fret == 0 { "X" } else { s });
        fretboard.push_str("-|");
        for i in 1..=MAX_FRET {
            fretboard.push_str(match (s, i) {
                _ if s == string && i == fret => "-X-|",
                ("A", 3 | 5 | 7 | 9 | 15 | 17 | 19 | 21) | ("D" | "E", 12 | 24) => "-o-|",
                _ => "---|",
            });
        }
        fretboard.push('\n');
    }
    fretboard
}

fn note_from_input() -> Result<String, io::Error> {
    let mut ret = String::new();
    io::stdin().read_line(&mut ret)?;
    Ok(ret.trim().to_string())
}

fn ask() -> bool {
    let string = random_choice(&STRINGS).unwrap_or(&"B");
    let fret = random_fret();
    println!("What note is this?");
    print!("{}", create_fretboard(string, fret));
    loop {
        print!("> ");
        if let Err(e) = io::stdout().flush() {
            println!("Error flushing stdout: {}", e);
            return false;
        }
        if let Ok(response) = note_from_input() {
            if let Ok(note) = note_from_string_and_fret(string, fret) {
                if response.is_empty() || response == "\n" {
                    println!("");
                    return false;
                }
                if note_equals(note, &response) {
                    println!("Correct.\n");
                    return true;
                }
            }
        }
        println!("Incorrect.\n");
    }
}

fn main() {
    while ask() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_from_string_and_fret() {
        assert_eq!(note_from_string_and_fret("B", 5).unwrap(), "E");
        assert_eq!(note_from_string_and_fret("E", 0).unwrap(), "E");
        assert_eq!(note_from_string_and_fret("A", 24).unwrap(), "A");
        assert_eq!(note_from_string_and_fret("D", 1).unwrap(), "D#");
        assert_eq!(note_from_string_and_fret("G", 16).unwrap(), "B");
    }

    #[test]
    fn test_note_equals() {
        assert!(note_equals("B", "B"));
        assert!(note_equals("A#", "Bb"));
        assert!(!note_equals("B", "C"));
        assert!(note_equals("E", "e"));
        assert!(note_equals("F#", "gb"));
    }

    #[test]
    fn test_create_fretboard() {
        assert_eq!(
            create_fretboard("B", 5),
            r#"G-|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
D-|---|---|---|---|---|---|---|---|---|---|---|-o-|---|---|---|---|---|---|---|---|---|---|---|-o-|
A-|---|---|-o-|---|-o-|---|-o-|---|-o-|---|---|---|---|---|-o-|---|-o-|---|-o-|---|-o-|---|---|---|
E-|---|---|---|---|---|---|---|---|---|---|---|-o-|---|---|---|---|---|---|---|---|---|---|---|-o-|
B-|---|---|---|---|-X-|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
"#
        );
    }
}
