use lazy_static::lazy_static;
use std::{cmp, collections::HashMap};

lazy_static! {
    static ref CHAR_WEIGHT: HashMap<char, f64> = {
        let mut map = HashMap::new();
        map.insert('0', 0.5);
        map.insert('1', 0.5);
        map.insert('2', 0.5);
        map.insert('3', 0.5);
        map.insert('4', 0.5);
        map.insert('5', 0.5);
        map.insert('6', 0.5);
        map.insert('7', 0.5);
        map.insert('8', 0.5);
        map.insert('9', 0.5);
        map.insert(' ', 0.25);
        map.insert('!', 0.333);
        map.insert('"', 0.555);
        map.insert('#', 0.5);
        map.insert('$', 0.5);
        map.insert('%', 1.0);
        map.insert('&', 0.83300006);
        map.insert('\'', 0.27800003);
        map.insert('(', 0.333);
        map.insert(')', 0.333);
        map.insert('*', 0.5);
        map.insert('+', 0.57000005);
        map.insert(':', 0.25);
        map.insert('-', 0.333);
        map.insert('.', 0.25);
        map.insert('/', 0.27800003);
        map.insert(',', 0.333);
        map.insert(';', 0.333);
        map.insert('<', 0.57000005);
        map.insert('=', 0.57000005);
        map.insert('>', 0.57000005);
        map.insert('?', 0.5);
        map.insert('@', 0.93000007);
        map.insert('A', 0.72200006);
        map.insert('B', 0.66700006);
        map.insert('C', 0.72200006);
        map.insert('D', 0.72200006);
        map.insert('E', 0.66700006);
        map.insert('F', 0.611);
        map.insert('G', 0.77800006);
        map.insert('H', 0.77800006);
        map.insert('I', 0.38900003);
        map.insert('J', 0.5);
        map.insert('K', 0.77800006);
        map.insert('L', 0.66700006);
        map.insert('M', 0.94400007);
        map.insert('N', 0.72200006);
        map.insert('O', 0.77800006);
        map.insert('P', 0.611);
        map.insert('Q', 0.77800006);
        map.insert('R', 0.72200006);
        map.insert('S', 0.55600005);
        map.insert('T', 0.66700006);
        map.insert('U', 0.72200006);
        map.insert('V', 0.72200006);
        map.insert('W', 1.0);
        map.insert('X', 0.72200006);
        map.insert('Y', 0.72200006);
        map.insert('Z', 0.66700006);
        map.insert('[', 0.333);
        map.insert('\\', 0.27800003);
        map.insert(']', 0.333);
        map.insert('^', 0.58100003);
        map.insert('_', 0.5);
        map.insert('`', 0.333);
        map.insert('a', 0.5);
        map.insert('b', 0.55600005);
        map.insert('c', 0.44400004);
        map.insert('d', 0.55600005);
        map.insert('e', 0.44400004);
        map.insert('f', 0.333);
        map.insert('g', 0.5);
        map.insert('h', 0.55600005);
        map.insert('i', 0.27800003);
        map.insert('j', 0.333);
        map.insert('k', 0.55600005);
        map.insert('l', 0.27800003);
        map.insert('m', 0.83300006);
        map.insert('n', 0.55600005);
        map.insert('o', 0.5);
        map.insert('p', 0.55600005);
        map.insert('q', 0.55600005);
        map.insert('r', 0.44400004);
        map.insert('s', 0.38900003);
        map.insert('t', 0.333);
        map.insert('u', 0.55600005);
        map.insert('v', 0.5);
        map.insert('w', 0.72200006);
        map.insert('x', 0.5);
        map.insert('y', 0.5);
        map.insert('z', 0.44400004);
        map.insert('{', 0.39400002);
        map.insert('|', 0.22000001);
        map.insert('}', 0.39400002);
        map.insert('~', 0.52000004);
        map
    };
}

/// caluclate height of cells from the content of cells and its font size
pub fn calculate_height(content: &mut String, size: &usize, cell_width: usize) -> usize {
    let mut cell_height: usize = 20;
    let mut curr_width: usize = 6;
    let mut temp_string = String::new();

    for i in content.chars() {
        let char_width = CHAR_WEIGHT
            .get(&i)
            .expect("character not present in hashmap")
            * (*size as f64);

        curr_width += char_width as usize;

        temp_string.push(i);

        if curr_width > cell_width {
            curr_width = 6;
            cell_height += 20;
            temp_string.push('\n');
        }
    }

    // Replace the content of content with temp_string
    *content = temp_string;
    cell_height
}
