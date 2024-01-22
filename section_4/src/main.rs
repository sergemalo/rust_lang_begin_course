mod player;
//use crate::archive::arch::arch_file;
use crate::archive::arch::arch_file as arc;

use rand::Rng;

mod archive;
fn main() {
    println!("Hello, world!");

    player::play_movie("The Movie.mp4");
    player::play_audio("name.wav");

    clean::perform_clean();
    clean::files::clean_files();

    arc("test.txt");

    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen();
    println!("Random number: {}", random_number);
}

mod clean {
    pub fn perform_clean() {
        println!("Cleaning up..."); 
    }

    pub mod files {
        pub fn clean_files() {
            println!("Cleaning files...");
        }
    }
}