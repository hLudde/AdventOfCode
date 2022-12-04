use crate::file_utils;

pub fn task(){
    let mut unfair_sectors = 0;
    if let Ok(lines) = file_utils::read_lines("./resources/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let mut sectors: Vec<u32> = Vec::new();
                for sector in ip.split(',') {
                    for section_id in sector.split('-'){
                        sectors.push(section_id.parse().unwrap())
                    }
                }
                let elf_one_sectors = vec![sectors[0],sectors[1]];
                let elf_two_sectors = vec![sectors[2],sectors[3]];
                if elf_one_sectors[0] <= elf_two_sectors[0] && elf_one_sectors[1] >= elf_two_sectors[1]{
                    unfair_sectors +=1 ; 
                    continue;
                }
                if elf_two_sectors[0] <= elf_one_sectors[0] && elf_two_sectors[1] >= elf_one_sectors[1]{
                    unfair_sectors +=1 ; 
                    continue;
                }
            }
        }
    }
    println!("Number of unfair sectors: {}", unfair_sectors)
}