
// fn main() {
// //    let width1=30;
// //    let height1=50;

// //    println!("The area of rectangle in pixels {}",area(width1,height1));

// /*Refactoring with tuples */

// let rect=(30,50);

// println!("The area of rect {}",area(rect))

// }

// // fn area(width:u32,height:u32)-> u32{
// //     width*height
// // }

// fn area(dimensions:(u32,u32)) -> u32{
//     dimensions.0*dimensions.1
// }

struct Rectangle{
    width:u32,
    height:u32
}
fn main(){
   let rect1 =Rectangle{
    width:30,
    height:50
   };

   println!("The area of rect is {}",area(&rect1));

}

fn area(rectangle: &Rectangle)-> u32{
    rectangle.width*rectangle.height
}