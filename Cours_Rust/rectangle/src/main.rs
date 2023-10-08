fn main() {
    let rect1 = (30, 50);
    
    println!(
        "L'air du rectangle est de {} pixel carré",
        air(rect1)
    );
}

fn air(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}