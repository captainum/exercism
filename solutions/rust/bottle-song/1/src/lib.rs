use std::collections::HashMap;

fn capitalize(s: &str) -> String {
    format!("{}{}", &s[..1].to_string().to_uppercase(), &s[1..])
}

fn prepare_suffix(idx: u32) -> String {
    if idx == 1 {
        String::from("")
    }
    else {
        String::from("s")
    }
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut result = String::new();

    let numbers = HashMap::from([
        (0, "no"),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
    ]);

    for i in 0..take_down {
        let first_idx = start_bottles - i;
        let second_idx = first_idx - 1;

        let first_number_str = numbers.get(&first_idx).unwrap();
        let second_number_str = numbers.get(&second_idx).unwrap();

        for _j in 0..2 {
            result.push_str(
                &format!("{} green bottle{} hanging on the wall,\n", capitalize(&first_number_str), prepare_suffix(first_idx))
            );
        }

        result.push_str("And if one green bottle should accidentally fall,\n");

        result.push_str(
            &format!("There'll be {} green bottle{} hanging on the wall.", second_number_str, prepare_suffix(second_idx))
        );

        if i != take_down - 1 {
            result.push_str("\n\n");
        }
    }

    result
}