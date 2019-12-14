
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

const IMG_X: usize = 25;
const IMG_Y: usize = 6;

fn main() {
    let i = ImageData::from_file("input");
    // println!("{:?}", &i.v);
    let image = &i.image;
    let mut max_seen = 0;
    let mut layer_max = 0;

    let mut message = [2; IMG_X*IMG_Y];
    for (i, layer) in image.iter().enumerate() {
        let mut nonzeros = 0;
        for (pixel, digit) in layer.iter().enumerate() {
            if *digit != 0 {
                nonzeros += 1;
            }
            if message[pixel] == 2 {
                message[pixel] = *digit;
            }
        }
        if nonzeros > max_seen {
            max_seen = nonzeros;
            layer_max = i;
        }
    }
    println!("Found layer {} to have fewest zeroes.", layer_max);

    let mut totals = [0, 0, 0];
    for d in &image[layer_max] {
        totals[*d as usize] += 1;
    }
    let (ones, twos) = (totals[1], totals[2]);
    println!("Layer {} has {} ones and {} twos", layer_max, ones, twos);
    println!("{} * {} = {}", ones, twos, ones*twos);

    for i in 0..IMG_Y {
        for j in 0..IMG_X {
            let printer = match message[i*IMG_X + j] {
                1 => Some(' '),
                0 => Some('#'),
                _ => None
            }.expect("");
            print!("{}", printer);
        }
        println!("");
    }
}

struct ImageData {
    image: Vec::<Vec::<i32>>,
}

impl ImageData {
    fn from_file(filename: &str)
        -> ImageData
    {
        let mut ret = ImageData {
            image: Vec::<Vec::<i32>>::new(),
        };
        let mut f = File::open(filename).expect("");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("");
        let mut i = contents.chars();
        loop {
            let mut layer_vec = Vec::<i32>::new();
            for _ in 0..IMG_X*IMG_Y {
                if let Some(c) = i.next() {
                    layer_vec.push(c.to_digit(10).expect("") as i32);
                } else {
                    ret.image.push(layer_vec);
                    return ret;
                }
            }
            ret.image.push(layer_vec);
        }
    }
}
