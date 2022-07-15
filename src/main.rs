//#![allow(unused)]

mod icons;
use icons::{get_image, resize};


const SIZES_ANDROID: [u32; 8] = [72, 96, 128, 144, 152, 192, 384, 512];
const SIZES_IOS: [u32; 2] = [120, 180];

fn main() {

//get args, remove first
    let mut pathes: Vec<String> = std::env::args().collect();
    pathes.drain(..1);

//concat 2 arrays in iterable
    let sizes: Vec<u32> = IntoIterator::into_iter(SIZES_ANDROID).chain(IntoIterator::into_iter(SIZES_IOS)).collect();
    
    for path in &pathes {
        
    //trunc extension if exists
        let parts = path.split(".").collect::<Vec<&str>>();

        let file_name = if parts.len()>=2 { parts.get(parts.len()-2).unwrap() } else { parts.get(0).unwrap() };
        println!("create icons for {}", file_name);

        match get_image(&path) {
            Ok(im) =>  {
                resize(&sizes, &im, &file_name);
            },
            Err(_) => {
                println!("failed to create image");
            }
        };
    }
}


