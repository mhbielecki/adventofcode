use std::fs;

// The image is 25 pixels wide and 6 pixels tall.
// Each layer is a 2D-array
// Picture is a list of layers
fn main() {
    let filename = "input/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");

    let pixels: Vec<i32> = contents
        .chars()
        .collect::<Vec<char>>()
        .into_iter()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect();
        
    let chunk_size = 25 * 6;
    let layers: std::slice::Chunks<i32> = pixels.chunks(chunk_size);
    let min = layers.clone().into_iter().min_by_key(|x| count_digit(&x, 0));

    //part 1: find the layer that contains the fewest 0 digits. On that layer, what is the number of 1 digits multiplied by the number of 2 digits?
    match min {
        Some(x) => {
            let ones = count_digit(&x,1);
            let twos = count_digit(&x,2);
            println!("Part 1: {}", ones * twos);
        },
        None => panic!("Error")
    };

    //Part 2 - Decode image and show the secret password!
    let mut final_layer: [[i32; 25]; 6] = [[2; 25]; 6];

    for layer in layers {
        let mut pixel_idx = 0;
        for x in 0..6 {
            for y in 0..25 {
                if final_layer[x][y] == 2   {
                    final_layer[x][y] = layer[pixel_idx];
                }
                pixel_idx = pixel_idx + 1;
            }
        }
    }

    for layer in final_layer.iter() {
        pretty_print(&layer);
    }
}

fn count_digit(layer: &[i32], digit: i32) -> usize {
    let digits: Vec<_> = layer.iter().filter(|&x| *x == digit).collect();
    digits.len()
}

fn pretty_print(layer: &[i32; 25]) {
    for c in layer.iter() {
        if *c == 0 {
            print!("~");
        } 
        else {
            print!("X");
        }
    }
    println!();
}
