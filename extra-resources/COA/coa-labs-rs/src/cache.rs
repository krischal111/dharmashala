// Cache replacement
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
static ref_string : &[i32]= &[1, 4, 0, 3, 4, 5, 6, 4, 7, 9];

// Code here is pretty hard to explain.
// So, let me try it some other day
fn FIFO(frame_size : usize) {
    let mut insert_at = 0;
    let mut cache = Vec::new();
    println!("Insert \tCache \t");
    for item in ref_string.iter() {
        let mut case = "";
        if cache.contains(item) {
            case = "hit";
        } else {
            case = "miss";
            if cache.len() < frame_size {
                cache.push(*item);
            } else {
                cache[insert_at] = *item;
            }
            insert_at += 1;
            insert_at %= frame_size;
        }
        println!("{item}\t{cache:?}\t{case}");
    }
}

fn LRU(frame_size : usize) {
    let mut cache = Vec::<i32>::new();
    let mut usage = Vec::<i32>::new();
    
    println!("Insert \tCache \t");
    for item in ref_string.iter() {
        let mut case = "";
        let position = cache.iter().position(|x| x == item);
        if let Some(position) = position {
            case = "hit";
            let usage_level = usage[position];
            for item in usage.iter_mut() {
                if *item < usage_level {
                    *item += 1;
                }
            }
            usage[position] = 0;
        } else {
            case = "miss";
            // if frame is full
            // we iterate over the frame's usage
            // and replace the one that was used oldest time
            // while all these time incrementing other's usage
            for (i, val) in usage.iter_mut().enumerate() {
                *val += 1;
                if *val as usize == frame_size {
                    *val = 0;
                    cache[i] = *item;
                }
            }
            // however, if the frame is not full
            // we have to insert it anew
            if cache.len() < frame_size {
                cache.push(*item);
                usage.push(0);
            }
        }
        let cache_usage:Vec<(i32, i32)> = cache.iter().copied().zip(usage.iter().copied()).collect();
        println!("{item}\t{cache_usage:?}\t{case}");
    }
}

#[test]
fn test_fifo() {
    FIFO(4);
    LRU(4);
}