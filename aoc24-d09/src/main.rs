use std::fs::read_to_string;

const DAY: u8 = 9;

fn main() {
    println!(
        "\n/* {:40} */ \n/* Day {:02}: {:32} */",
        "Advent of Code 2024", DAY, "Disk Fragmenter"
    );
    let data = read_to_string("input.txt").unwrap();

    let start = std::time::Instant::now();
    let (part1, part2) = solve(&data);
    let time = start.elapsed();

    println!("Checksum with fragmented files (Part 1): {part1}");
    println!("Checksum without fragmented files (Part 2): {part2}");

    let ns = time.as_nanos();
    let version = rustc_version::version().unwrap();
    let lines = read_to_string("src/main.rs").unwrap().lines().count();
    let os_arch = format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH);
    println!("\n| Day {DAY} | \u{1F980} Rust {version} | \u{23F1}\u{FE0F} {time:?} ({ns} ns) | \u{1F4DC} {lines} lines | \u{2699}\u{FE0F} {os_arch} |");
}

fn solve(input: &str) -> (usize, usize) {
    let part1 = move_file_blocks_to_front(input);
    let part2 = move_complete_files_to_front(input);
    (part1, part2)
}

enum DiskBlock {
    Free,
    File(usize), //usize = file ID
}

fn move_file_blocks_to_front(input: &str) -> usize {
    // Parse as one long set of File-blocks and Free-blocks spaces
    let mut disk = Vec::with_capacity(input.len() * 9);
    for (index, number) in input.char_indices() {
        let number = number.to_digit(10).unwrap();
        if index % 2 == 0 {
            for _ in 0..number {
                disk.push(DiskBlock::File(index / 2))
            }
        } else {
            for _ in 0..number {
                disk.push(DiskBlock::Free)
            }
        }
    }

    // Loop over it, we are working from the outside to the center:
    let mut i_insert = 0;
    let mut i_end = disk.len() - 1;

    while i_insert < i_end {
        // skip when there is nothing to do:
        if let DiskBlock::File(_) = disk[i_insert] {
            i_insert += 1;
            continue;
        }
        if let DiskBlock::Free = disk[i_end] {
            i_end -= 1;
            continue;
        }

        // let's move the file at the end to the free space at the front
        let removed = disk.remove(i_end);
        disk[i_insert] = removed;
        i_insert += 1;
        i_end -= 1;
    }

    calculate_checksum_on_blocks(disk.as_slice())
}

fn calculate_checksum_on_blocks(disk: &[DiskBlock]) -> usize {
    let mut checksum = 0;
    for i in 0..disk.len() {
        match &disk[i] {
            DiskBlock::File(id) => checksum += id * i,
            _ => {}
        }
    }
    checksum
}

enum DiskFragment {
    Free(u8),        // u8 = size
    File(usize, u8), //usize = file ID, u8 = size
}

fn move_complete_files_to_front(input: &str) -> usize {
    // Parse it as a set of fragments, each contains its own size.
    let mut disk = Vec::with_capacity(input.len() * 2);
    for (index, number) in input.char_indices() {
        let size = number.to_digit(10).unwrap() as u8;
        if index % 2 == 0 {
            let id = index / 2;
            disk.push(DiskFragment::File(id, size))
        } else {
            disk.push(DiskFragment::Free(size))
        }
    }

    // Loop over the files from end to beginning.
    for i in (0..disk.len()).rev() {
        if let DiskFragment::File(_, file_size) = disk[i] {
            // time to find a spot to move the file to, loop from start to find first space that fits
            'replace: for j in 0..i {
                if let DiskFragment::Free(free_size) = disk[j] {
                    if free_size >= file_size {
                        // overwrite free space & put a free space back at old location:
                        disk[j] = disk.remove(i);
                        disk.insert(i, DiskFragment::Free(file_size));
                        if file_size < free_size {
                            // when there was room over, put some remaining free space
                            disk.insert(j + 1, DiskFragment::Free(free_size - file_size));
                        }
                        break 'replace;
                    }
                    // continue searching, not enough space to fit
                }
            }
        }
    }

    calculate_checksum_on_fragments(disk.as_slice())
}

fn calculate_checksum_on_fragments(disk: &[DiskFragment]) -> usize {
    let mut checksum: usize = 0;
    let mut i: usize = 0;
    for file in disk {
        match file {
            DiskFragment::File(id, size) => {
                for _ in 0..*size {
                    checksum += id * i;
                    i += 1;
                }
            }
            DiskFragment::Free(size) => {
                i += *size as usize;
            }
        }
    }
    checksum
}
