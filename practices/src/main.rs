use image::GenericImageView;

fn main() {
    let img = image::open("practices/sources/test.png").expect("Could not load image");
    let pixel = img.get_pixel(47, 54);
    let (r, g, b, a) = (pixel[0], pixel[1], pixel[2], pixel[3]);

    println!("r: {}, g: {}, b: {}, a: {}", r, g, b, a);
}
