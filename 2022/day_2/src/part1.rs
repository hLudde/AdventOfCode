use crate::file_utils;

pub fn task() {
    if let Ok(lines) = file_utils::read_lines("./resources/input.txt") {
        let mut sum = 0;
        for line in lines {
            if let Ok(ip) = line {
                let mut score = 0;
                let hands: Vec<&str> = ip.split(" ").collect();
                let elf = hands[0];
                let player = hands[1];
                if player.eq("X") {
                    score += 1;
                    if elf.eq("A") {
                        score += 3;
                    }
                    if elf.eq("B") {
                        score += 0;
                    }
                    if elf.eq("C") {
                        score += 6;
                    }
                }
                if player.eq("Y") {
                    score += 2;
                    if elf.eq("A") {
                        score += 6;
                    }
                    if elf.eq("B") {
                        score += 3;
                    }
                    if elf.eq("C") {
                        score += 0;
                    }
                }
                if player.eq("Z") {
                    score += 3;
                    if elf.eq("A") {
                        score += 0;
                    }
                    if elf.eq("B") {
                        score += 6;
                    }
                    if elf.eq("C") {
                        score += 3;
                    }
                }
                sum += score;
            }
        }
        println!("Part one: {}", sum);
    }
}