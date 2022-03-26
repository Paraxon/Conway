use std::fs;

fn main() ->std::io::Result<()> {
    let mut file = fs::File::create("img.ppm").unwrap();
    let width = 3;
    let height = 2;
    fs::write("img.ppm", "P3\n3 2\n255\n255 0 0 0 255 0 0 0 255\n255 255 0 255 255 255 0 0 0");
    Ok(())
}
