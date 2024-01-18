use snappy::snappy_sys;

fn main() {
    let x = unsafe { snappy_sys::snappy_max_compressed_length(100) };
    println!("max compressed length of a 100 byte buffer: {}", x);
}
