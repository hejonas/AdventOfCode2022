use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut cur: u32 = 0;
    let mut elvs = Vec::new();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    for line in input.lines() {
        if line.is_empty(){
            elvs.push(cur);
            cur = 0;
        }
        else {
            cur = cur + line.parse::<u32>().expect("Failed to parse");
        }
    }

    // Check max
    let max_value = elvs.iter().max();
    println!("Top elf is carrying {} calories", max_value.unwrap());
    
    // Check top 3 max
    elvs.sort_by(|a, b| b.cmp(a));
    println!("Top 3 elvs are carrying {} calories", elvs.iter().take(3).sum::<u32>());

    Ok(())
}
