trait Area{
    fn area(&self) -> f64;
}

struct Square<T>{
    area_value : f64,
    side : T,
}

impl Square<u32> {
    fn new(param : u32) -> Self{
        return Square::<u32>{ side: param, area_value : (param*param) as f64};
    }
}

impl Square<f64> {
    fn new(param : f64) -> Self{
        return Square::<f64>{ side: param, area_value : (param*param) as f64};
    }
}

impl Square<String> {
    fn new(param : &str) -> Self{
        let side_value = param.parse::<f64>().unwrap();
        return Square::<String>{ side: String::from(param), area_value : side_value*side_value};
    }
}

impl<T> Area for Square<T>{
    fn area(&self) -> f64{
        return self.area_value;
    }
}

struct Triangle{
    a : f64,
    b : f64,
    c : f64
}

impl  Triangle {
    fn new(_a : f64,_b : f64,_c : f64) -> Self { 
        return Triangle{a: _a, b: _b, c: _c };
    }
}
 impl Area for Triangle{
    fn area(&self) -> f64{
        let s = (self.a+self.b+self.c) / 2.0;
        return ( s*(s-self.a)*(s-self.b)*(s-self.c) ).sqrt();
    }
 }

struct Pyramid<T,Y>{
    base : T,
    height : Y,

}
impl<T,Y> Pyramid<T, Y>{
    fn new(s : T, h : Y) -> Self {
        return Pyramid { base: s, height: h }
    }
    fn volume(&self) -> f64{
        return self.base.area() * self.height as f64;
    }
}

fn main() {
    let square = Square::<u32>::new(5);
    let square_float = Square::<f64>::new(5.4);
    let square_string = Square::<String>::new("6");

    println!("square area is {}", square.area());
    println!("square_float area is {}", square_float.area());
    println!("square_string area is {}", square_string.area());

    let triangle = Triangle::new(14.9, 20.1, 16.0);
    let pyramid_square = Pyramid::<Square<u32>, f64>::new(square, 24.3);
    let pyramid_triangle = Pyramid::<Triangle, f64>::new(triangle, 24.3);

    println!("pyramid_square volume is {}", pyramid_square.volume());
    println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
}
