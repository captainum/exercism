static ASCII: [char; 26] = [
    'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y',
    'Z',
];

pub fn get_diamond(c: char) -> Vec<String> {
    let mut result = Vec::<String>::new();

    let counter = (c as usize) - ('A' as usize);

    for (i, val) in ASCII.iter().enumerate().take(counter + 1) {
        let mut line = Vec::new();
        line.push(" ".repeat(counter - i));
        line.push(val.to_string());
        if i > 0 {
            line.push(" ".repeat(i - 1));
            line.push(" ".to_string());
        }

        line.append(
            &mut line.iter().rev().skip(1).cloned().collect::<Vec<String>>()
        );

        result.push(line.join(""));
    }

    result.append(
        &mut result.iter().rev().skip(1).cloned().collect::<Vec<String>>()
    );

    result
}