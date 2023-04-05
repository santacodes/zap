use rand::Random;

fn genotp(){
    //Generate and return a 4 digit number 
    let x = rand::random::<i64>(1000,9999);
    return x;
}