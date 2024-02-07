/// Calculates the frequency of each character in a vector and returns a vector of tuples
pub fn get_frequency(data: Vec<char>) -> Vec<(char, i32)> {
    let mut frequency: Vec<(char, i32)> = Vec::new();

    for &ch in &data {
        let mut found = false;

        for i in &mut frequency {
            if i.0 == ch {
                i.1 += 1;
                found = true;
                break;
            }
        }

        if !found {
            frequency.push((ch, 1));
        }
    }

    frequency
}

/// Merges two vectors of character frequencies into a tuple of two vectors.
pub fn merge_frequency(first: &mut Vec<(char, i32)>, second: &mut Vec<(char, i32)>) -> (Vec<(char, i32)>, Vec<(char, i32)>) {
    let mut common: Vec<(char, i32)> = Vec::new();
    let mut left: Vec<(char, i32)> = Vec::new();

    let mut i = 0;
    while i < first.len() {
        let mut j = 0;
        let mut found = false;

        while j < second.len() {
            if first[i].0 == second[j].0 {
                common.push((first[i].0, first[i].1 + second[j].1));
                second.remove(j);
                found = true;
                break;
            }
            j += 1;
        }

        if !found {
            left.push(first[i]);
        }

        i += 1;
    }
    for i in second {
        left.push(*i);
    }

    (common, left)
}


/// Sorts a vector of character frequencies based on the characters.
pub fn sort_freq(data: Vec<(char, i32)>) -> Vec<(char, i32)> {
    let mut sorted_data = data;

    let len = sorted_data.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if sorted_data[j].0 > sorted_data[j + 1].0 {
                sorted_data.swap(j, j + 1);
            }
        }
    }

    sorted_data
}


/// Fits characters with frequencies into a vector of characters based on the provided string.
pub fn fit_into_string(mut str: Vec<char>, common_fr: &mut Vec<(char, i32)>) -> String {
    let mut j = 1;

    for i in 0..str.len() {
        if str[i] == '_' && j < common_fr.len() {
            let (ch, count) = &mut common_fr[j];

            if *count > 0 {
                str[i] = *ch;
                *count -= 1;

                if *count == 0 {
                    j += 1;
                }
            }
        }
    }

    str.into_iter().collect()
}
