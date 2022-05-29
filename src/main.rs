fn main() {
    let a = Color::new(8, 1,8);
    let b: Color = Color::new(255,0,0);
    let c: Color = Color::add_color(&a, &b);
    println!("Hello, world!");
    println!("A: {},{},{}",a.r,a.b,a.g);
    println!("B: {},{},{}",b.r,b.b,b.g);
    println!("C: {},{},{}",c.r,c.b,c.g);
}

#[derive(Debug)]
struct Color{
    r: u8,
    g: u8,
    b: u8
}

impl Color{

    fn new(r: u8, g: u8, b: u8) -> Color{
        Color{r, g, b}
    }

    fn add_color(&self, p_color: &Color) -> Color {
        let new_r = u8::saturating_add(self.r, p_color.r);
        let new_g = u8::saturating_add(self.g, p_color.g);
        let new_b = u8::saturating_add(self.b, p_color.b);

        Color{r: new_r, g: new_g, b: new_b}
    }
}

#[test]
fn add_test(){
    let result: u8 = 2 + 2;
    assert_eq!(result, 4);
}