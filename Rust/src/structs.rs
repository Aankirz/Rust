struct Student{
    name:String,
    age:u32,
    roll_no:u32,
   is_present:bool,
}


pub fn run(){
  

  let mut student_1=Student{
    name:String::from("Ankit"),
    age:19,
    roll_no:1,
    is_present:true,

  };

  student_1.name=String::from("Ankit Kiran");


  /* Struct Update Syntax */
  let mut student_2=Student{
    name:String::from("Kiran"),
    roll_no:2,
    ..student_1
  };
}

fn build_student(name:String, age:u32)-> Student{
      Student { name, age, roll_no: 10, is_present: true }
}



// Chapter 10