use rand;
//use std::hash;

pub fn genotp() -> i32 {
    //Generate and return a 4 digit number
    use rand::Rng; // 0.8.5

    // Generate random number in the range [0, 99] to generate otp 
    let num = rand::thread_rng().gen_range(1000..9999);
    println!("This is the generated random number {}", num);
    return num;
}
pub fn verify(otp: i32) -> bool {
    let databaseotp = 2123;  //some random number for test
    if otp == databaseotp {
        println!("Verified the OTP");
        return true;

    }else{
        print!("Not Verified the OTP");
        return false;
    }
    
 }