///add "_" at the poistion where the string have character we add in example 'e'
pub fn cut() -> Vec<char> {
    let str = "Welcome to inventyv software services";
    let mut str: Vec<char> = str.to_lowercase().chars().collect();
    let char = 'e';

    for i in 0..str.len() {
        if str[i] == char {
            str[i] = '_'
        }
    }
    str
}
