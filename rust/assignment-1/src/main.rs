#[derive(Debug)]
struct Shape {
    length: f32,
    breath: f32,
}

impl Shape {
    fn new(length: f32, breath: f32) -> Shape {
        Shape { length, breath }
    }

    fn rectangle(&self) -> f32 {
        self.length * self.breath
    }

    fn square(&self) -> f32 {
        self.length * self.length
    }

    fn circle(&self) -> f32 {
        if self.length > self.breath { 3.14 * (self.breath / 2.0) } else { 3.14 * (self.length / 2.0) }
    }
    
}

fn main() {
    let shape = Shape::new(12.0, 12.0);
    println!("{:?}", shape);

    println!("{}" , shape.rectangle());
    println!("{}" , shape.square());
    println!("{}" , shape.circle());
}
