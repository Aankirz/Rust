#[derive(Debug)]
struct Rect{
    height:u32,
    width:u32
}

impl Rect{
    fn area(&self)-> u32{
        &self.height*&self.width
    }
}



fn main() {
   let height1=50;
   let width1=30;

   let rect_1=(50,30); /* STACK MEMORY */

   let rect_2=Rect{        /* HEAP MEMORY */
    height:height1,
    width:width1,
   };
  
   println!("rect_2 is {:#?}", &rect_2);

   
//    println!("The area of the rectangle is {} square pixels.", area(&rect_2));
    println!("The area of the rectangle is {} square pixels.", rect_2.area());
}

// fn area(rect:(u32,u32))->u32{
//    rect.0*rect.1
// }

// fn area(rectangle:&Rect)-> u32{
//     rectangle.height*rectangle.width
// }