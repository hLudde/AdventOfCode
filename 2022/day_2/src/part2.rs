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
                if elf.eq("A") {
                    if player.eq("X") {
                        score += 3+0;
                    }
                    if player.eq("Y") {
                        score += 1+3;
                    }
                    if player.eq("Z") {
                        score += 2+6;
                    }
                }
                if elf.eq("B") {
                    if player.eq("X") {
                        score += 1+0;
                    }
                    if player.eq("Y") {
                        score += 2+3;
                    }
                    if player.eq("Z") {
                        score += 3+6;
                    }
                }
                if elf.eq("C") {
                    if player.eq("X") {
                        score += 2+0;
                    }
                    if player.eq("Y") {
                        score += 3+3;
                    }
                    if player.eq("Z") {
                        score += 1+6;
                    }
                }
                sum += score;
            }
        }
        println!("Part two: {}", sum);
    }
}