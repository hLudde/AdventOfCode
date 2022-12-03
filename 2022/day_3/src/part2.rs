use crate::file_utils;

pub fn task(){
    if let Ok(lines) = file_utils::read_lines("./resources/input.txt") {
        let mut sum = 0;
        let mut elf_backpacks: Vec<String> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                elf_backpacks.push(ip.clone());
                if elf_backpacks.len() % 3 == 0 {
                    let mut found_char = false;
                    for char_backpack_one in elf_backpacks[0].chars() {
                        for char_backpack_two in elf_backpacks[1].chars() {
                            if char_backpack_one == char_backpack_two {
                                for char_backpack_three in elf_backpacks[2].chars(){
                                    if char_backpack_one == char_backpack_three {
                                        sum += get_char_priority(char_backpack_one);
                                        found_char = true;
                                    }
                                    if found_char {
                                        break;
                                    }
                                }
                            }
                            if found_char {
                                break;
                            }
                        }
                        if found_char {
                            break;
                        }
                    }
                    elf_backpacks = Vec::new();
                }
            }
        }
        println!("Badge priority: {}", sum);
    }
}

fn get_char_priority(character: char) -> u32{
    let ascii_code = character as u32;
    if ascii_code < 96 {
        return ascii_code-38;
    }
    else {
        return ascii_code-96;
    }
}