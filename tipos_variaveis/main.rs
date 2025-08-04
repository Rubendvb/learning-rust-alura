const PI:f32 = 3.14159265358979323846;
static mut GLOBAL:u8 = 1;

fn main() {
    println!("PI = {}", PI);

    unsafe {
        println!("GLOBAL = {}", GLOBAL);
    }
}