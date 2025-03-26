use rand::Rng;

fn main() {

    let tmin: i32;
    let tmax: i32;
    let args: Vec<String> = std::env::args().collect();

if args.len() >= 4 {
    let min = &args[2];
    let max = &args[4];

    tmin = min.parse::<i32>().unwrap();
    tmax = max.parse::<i32>().unwrap();
}
else {
    tmin = 0;
    tmax = 100;
}
    
    let mut rng = rand::rng();
    if tmin > tmax {
        println!("Your Number is: {}", rng.random_range(tmax..=tmin))
    }
    else if tmax > tmin {
        println!("Your Number is {}", rng.random_range(tmin..=tmax))
    }
}
