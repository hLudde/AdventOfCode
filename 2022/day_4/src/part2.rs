use crate::file_utils;

pub fn task(){
    let mut overlapping_sectors = 0;
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
                for sector_id_one in elf_one_sectors[0]..elf_one_sectors[1]+1 {
                    let mut found_sector = false;
                    for sector_id_two in elf_two_sectors[0]..elf_two_sectors[1]+1 {
                        if sector_id_one == sector_id_two {
                            overlapping_sectors += 1;
                            println!("Found sector! {}", sector_id_one);
                            found_sector = true;
                            break;
                        }
                    }
                    if found_sector {
                        break;
                    }
                }
            }
        }
    }
    println!("Number of Overlapping sectors: {}", overlapping_sectors)
}