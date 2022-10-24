use std::str::FromStr;

trait Area{
    fn area(&self) -> f64;
}

struct Square{
    side : f64,
}

impl Square {
    fn new<T : TryInto<f64>>(param : T ) -> Self
    {
        let _side = param.try_into().unwrap_or(0.0);
        return Square{ side: _side, };
    }
}

impl Area for Square{
    fn area(&self) -> f64{
        return self.side * self.side;
    }
}
struct Triangle{
    a : f64,
    b : f64,
    c : f64
}

impl Triangle {
    fn new<T : TryInto<f64>>(_a : T,_b : T,_c : T) -> Self { 
        return Triangle{a: _a.try_into().unwrap_or(0.0), b: _b.try_into().unwrap_or(0.0), c: _c.try_into().unwrap_or(0.0) };
    }
}
 impl Area for Triangle{
    fn area(&self) -> f64{
        let s = (self.a+self.b+self.c) / 2.0;
        return ( s*(s-self.a)*(s-self.b)*(s-self.c) ).sqrt();
    }
 }

#[derive(Clone,Copy)]
struct Pyramid<T,Y>{
    base : T,
    height : Y,

}
impl<T : Area,Y : TryInto<f64>+Copy> Pyramid<T,Y>{
    fn new(s : T, h : Y) -> Self {
        return Pyramid { base: s, height: h }
    }
    fn volume(&self) -> f64{
        let x : f64  = self.base.area();
        let h : f64 = self.height.try_into().unwrap_or(0.0);
        return x*h;
    }
}

fn main() {
    let square = Square::new::<u32>(5);
    let square_float = Square::new::<f64>(5.4);

    println!("square area is {}", square.area());
    println!("square_float area is {}", square_float.area());

    let triangle = Triangle::new(14.9, 20.1, 16.0);
    let pyramid_square = Pyramid::<Square, f64>::new(square, 24.3);
    let pyramid_triangle = Pyramid::<Triangle, f64>::new(triangle, 24.3);

    println!("pyramid_square volume is {}", pyramid_square.volume());
    println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
}
