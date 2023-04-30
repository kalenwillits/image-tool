pub fn crop(infile: String, outfile: String, x: u32, y: u32, w: u32, h: u32) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    img.crop(x, y, w, h);
    img.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn rotate(infile: String, outfile: String, amount: i32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    match amount {
        90 => {
            let img2 = img.rotate90();
            img2.save(outfile).expect("Failed writing OUTFILE.");
        }
        180 => {
            let img2 = img.rotate180();
            img2.save(outfile).expect("Failed writing OUTFILE.");
        }
        270 => {
            let img2 = img.rotate270();
            img2.save(outfile).expect("Failed writing OUTFILE.");
        }
        _ => {
            println!("Invalid amount, rotation amounts are 90, 180, or 270");
        }
    }
}
