use std::f64::consts::PI;

// 假设
pub trait  Shape {
    fn name(&self) -> &str;
    fn area(&self) -> f64;
}
pub struct Rectangle {
    pub w: f64,
    pub h: f64,
}
impl  Rectangle{
    pub fn new(w: f64, h: f64) -> Rectangle {
        Rectangle{w, h}
    }
}
impl Shape for Rectangle {
    fn name(&self) -> &str {
        "Rectangle"
    }
    fn area(&self) -> f64 {
        self.w * self.h
    }
}
pub struct Circle {
    pub r: f64
}
impl Circle{
    pub fn new(r: f64) -> Circle {
        Circle{r}
    }
}
impl Shape for Circle {
    fn name(&self) -> &str {
        "Circle"
    }
    fn area(&self) -> f64 {
        PI * self.r * self.r
    }
}

pub fn  calculate_all_shape_area(shape: &[&dyn Shape]) -> f64 {
    shape.iter().map(|s| s.area()).sum()
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_rectangle_area(){
        let rectangle = Rectangle{w:2.0, h:3.0};
        assert_eq!(rectangle.area(), 6.0);
    }
    #[test]
    fn test_circle_area(){
        let circle = Circle{r:2.0};
        assert_eq!(circle.area(), PI * 2.0 * 2.0);
    }
    #[test]
    fn test_calculate_all_shape_area(){
        let rectangle = Rectangle{w:2.0, h:3.0};
        let circle = Circle{r:2.0};
        let shapes:Vec<&dyn Shape> = vec![&rectangle, &circle];
        assert_eq!(calculate_all_shape_area(&shapes), 2.0 * 3.0 + PI * 2.0 * 2.0);
    }
}








