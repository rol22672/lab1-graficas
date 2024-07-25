mod scanline;
mod polygon1;
mod polygon2;
mod polygon3;
mod polygon4;


fn main() {
    println!("Drawing Polygon 1...");

    polygon1::draw();

    println!("Drawing Polygon 2...");
    polygon2::draw();

    println!("Drawing Polygon 3...");
    polygon3::draw();

    println!("Drawing Polygon 4...");
    polygon4::draw();

}
