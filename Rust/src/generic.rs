
fn largest<T>(list:&[T])-> &T{
    let mut largest=&list[0];

    for item in list{
        if item > largest{  // Since we want to compare so it won't work for all values 
            largest=item;
        }

    }
    largest
}

/* vector element -> dereferencing (&) */


/*In struct */
struct Point<T1,U1>{
    x: T1,
    y: U1,
}

enum Options<T>{
     Some(T),
     none,
}

enum Result<T,E>{
    Ok(T),
    Err(E),
}

impl<T1,U1> Point<T1,U1>{
    fn mixup<T2,U2>(self,other:Point<T2,U2>)-> Point<T1,U2>{
        Point{
            x:self.x,
            y:other.y,
        }
    }
}

pub fn run(){
  let num_list=vec![20,30,40,50,60,70,80,90,100];
  
  let result=largest(&num_list);

  let p1=Point{x:5,y:10.4};
  
    let p2=Point{x:"Hello",y:'c'};

    let p3=p1.mixup(p2);

}