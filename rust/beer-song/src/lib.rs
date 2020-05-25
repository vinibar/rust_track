pub fn verse(n: u32) -> String {
    let next: u32 = if n as i32 - 1 < 0 { 99 } else { n - 1 };
    let aux: String = if n == 1 {
        String::from("it")
    } else {
        String::from("one")
    };

    let first_sentence: String = format!("{} of beer on the wall, {} of beer.\n", to_title(&format_qtd(&n)), format_qtd(&n));
    let last_sentence: String = if n >= 1 {
        format!("Take {} down and pass it around, {} of beer on the wall.\n", aux, format_qtd(&next))
    } else {
        String::from("Go to the store and buy some more, 99 bottles of beer on the wall.\n")
    };
    
    format!("{}{}", first_sentence, last_sentence)
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song: String = String::new();
    for i in (end..start + 1).rev() {
        song.push_str(&verse(i)[..]);
        if i > end {
            song.push_str("\n");
        }
    }
    song
}

fn format_qtd(qtd: &u32) -> String {
    let mut result = if qtd == &0 {
        String::from("no more bottle")
    } else {
        format!("{} bottle", qtd)
    };
    if qtd != &1 {
        result.push_str("s");
    }
    result
}

fn to_title(word: &String) -> String {
    let mut c = word.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}
