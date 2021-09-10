use std::time::SystemTime; 

// i made my own random function since i really didnt want to add a bunch of extra dependencies just for a rand num. seems good enough lol. 
/*
    this rng is a lot better than the previous one since it uses some PRNG algorithm i found on geeksforgeeks 
    now uses seed as an input. it is encouraged (by me) that some global counter variable is used for the seed or the 'i' in a for in loop. resetting the counter after some large number (something like 10,000) 
*/

pub fn random_num(seed: i32) -> u128 {
    let sys_time = SystemTime::now(); 
    match sys_time.duration_since(SystemTime::UNIX_EPOCH)  {
        Ok(n) => {
            let increment = 405; 
            let multiplier = 291; 
            let modulus = 112; 

            (( ((multiplier * seed + increment)%modulus)) as u128) * n.as_micros()
        }, 
        Err(_) => panic!("Calling time failed"), 
    }
}
pub fn rand_sequence(n: i32, seed:i32) -> Vec<u128> {
    let mut v: Vec<u128> = Vec::new(); 
    for i in 0..n {
        v.push(random_num(i + seed)); 
    }
    v
}

pub fn rand_from_range(range: [i32; 2], seed: i32) -> i128 {
    if range[1]-range[0] == 1 {
        (range[0] + (random_num(seed)%2) as i32) as i128
    } else {
        (range[0] + (random_num(seed)%((range[1]-range[0]) as u128) + random_num(seed+1)%2) as i32) as i128
    }
}

pub fn random_float(seed: i32) -> f32 {  // random float from [0,1)
    let sys_time = SystemTime::now(); 
    match sys_time.duration_since(SystemTime::UNIX_EPOCH)  {
        Ok(n) => {
            let increment = 405; 
            let multiplier = 291; 
            let modulus = 112; 

            let result = (( ((multiplier * seed + increment)%modulus)) as u128) * n.as_micros(); 

            match ("0.".to_string() + result.to_string().as_ref()).parse()  {
                Ok(v) => v, 
                Err(v) => panic!("Error for random_float string parse to f32 {}", v)
            }
            
        }, 
        Err(_) => panic!("Calling time failed"), 
    }
}

pub fn rand_float_from_range(range: [f32; 2], seed: i32) -> f32 {
    range[0] + random_float(seed)*(range[1]-range[0])
}