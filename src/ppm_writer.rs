pub struct Size {
    x: u16,
    y: u16,
}

pub fn write(size: &Size) ->std::io::Result<()> {
    for y in 0..size.y {
        for x in 0..size.x {
            if x == y {
                println!("255 255 255 ")} else {
                println!("0 0 0 ")
            }}
    }
    Ok(())
}
