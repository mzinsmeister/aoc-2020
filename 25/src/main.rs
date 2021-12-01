fn main() {
    // Demo Input
    //let public_key_1 = "5764801".parse::<u64>().unwrap();
    //let public_key_2 = "17807724".parse::<u64>().unwrap();
    // Puzzle Input
    let public_key_1 = "11239946".parse::<u64>().unwrap();
    let public_key_2 = "10464955".parse::<u64>().unwrap();
    let loop_size_1 = find_loop_size(public_key_1, 7);
    let loop_size_2 = find_loop_size(public_key_2, 7);
    println!("c: {}, d: {}", loop_size_1, loop_size_2);
    let mut encryption_key = 1;
    for _ in 0..loop_size_2 {
        encryption_key = (encryption_key * public_key_1) % 20201227;
    }
    println!("encryption key: {}", encryption_key);
}

fn find_loop_size(public_key: u64, subject_number: u64) -> u32 {
    let mut value = 1;
    let mut loop_counter = 0u32;
    while value != public_key {
        value = (value * subject_number) % 20201227;
        loop_counter += 1;
    }
    loop_counter
}
