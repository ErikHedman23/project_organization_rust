pub mod shapes;

use shapes::Feature;
use shapes::{circ, rect};

fn main() {
    let rect = rect::Rectangle::new(1.0, 2.0);
    let area = rect.get_feature(Feature::Area);
    println!("rect area is {}", area);

    let circ = circ::Circle::new(3.0);
    let perimeter = circ.get_feature(Feature::Perimeter);
    println!("circ perimeter is {}", perimeter)
}

//How to organize it all in one file:

// mod shapes {
//     pub enum Feature {
//         Area,
//         Perimeter,
//     }
//     pub mod rect {
//         pub struct Rectangle {
//             pub width: f64,
//             pub height: f64,
//         }

//         impl Rectangle {
//             pub fn new(width: f64, height: f64) -> Rectangle {
//                 return Rectangle { width, height };
//             }

//             pub fn get_feature(&self, feature: super::Feature) -> f64 {
//                 match feature {
//                     super::Feature::Area => self.calc_area(),
//                     super::Feature::Perimeter => self.calc_perimeter(),
//                 }
//             }

//             fn calc_area(&self) -> f64 {
//                 self.width * self.height
//             }

//             fn calc_perimeter(&self) -> f64 {
//                 2.0 * self.width + self.height
//             }
//         }
//     }

//     pub mod circ {
//         pub struct Circle {
//             pub radius: f64,
//         }

//         impl Circle {
//             pub fn new(radius: f64) -> Circle {
//                 Circle { radius }
//             }

//             pub fn get_feature(&self, feature: super::Feature) -> f64 {
//                 match feature {
//                     super::Feature::Area => self.calc_area(),
//                     super::Feature::Perimeter => self.calc_circumference(),
//                 }
//             }

//             fn calc_area(&self) -> f64 {
//                 std::f64::consts::PI * self.radius.powi(2)
//             }

//             fn calc_circumference(&self) -> f64 {
//                 2.0 * std::f64::consts::PI * self.radius
//             }
//         }
//     }
// }
