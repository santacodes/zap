use rand;

pub fn genotp() -> i32 {
    //Generate and return a 4 digit number
    use rand::Rng; // 0.8.5

    // Generate random number in the range [0, 99]
    let num = rand::thread_rng().gen_range(1000..9999);
    println!("This is the generated random number {}", num);
    return num;
}
