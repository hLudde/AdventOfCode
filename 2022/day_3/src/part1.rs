use crate::file_utils;


pub fn task(){
    if let Ok(lines) = file_utils::read_lines("./resources/input.txt") {
        let mut sum = 0;
        for line in lines {
            if let Ok(ip) = line {
                let compartment_one = &ip[..ip.len()/2];
                let compartment_two = &ip[ip.len()/2..ip.len()];
                for char_compartment_one in compartment_one.chars() {
                    let mut found_char = false;
                    for char_compartment_two in compartment_two.chars() {
                        if char_compartment_one == char_compartment_two {
                            let ascii_code = char_compartment_one as u32;
                            if ascii_code < 96 {
                                sum += ascii_code-38;
                            }
                            else {
                                sum += ascii_code-96;
                            }
                            found_char = true;
                        }
                        if found_char {
                            break;
                        }
                    }
                    if found_char {
                        break;
                    }
                }
            }
        }
        println!("Priority sum: {}", sum)
    }
}