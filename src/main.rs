mod scanline;
mod polygon1;
mod polygon2;

fn main() {
    println!("Drawing Polygon 1...");

    polygon1::draw();

    println!("Drawing Polygon 2...");
    polygon2::draw();

}
