fn void() { //void function
    println!("abc");
}
fn fint() -> i32 { //int function return
    return 5;
}
fn fint2() -> i32 { //int function
    5
}

fn main() {
    let int:i32 = 10; //int type
    let string:&str = "Hello"; //str type
    let c:char = 'a'; //char type
    //void(); //call function
    //println!("{}",fint()); //print function
    let pair:(char,i32)=('a',1); //tuples(pair)
    //println!("{}",pair.0); //first element
    //println!("{}",pair.1); //second element
    let (firstchar,secondchar) = ('a','b'); //split an tuple in 2 parts
    /*let x="hello"; //blocks
    {
        let x="HELLO";
        println!("{}",x);
    }
    println!("{}",x);*/
    /*if 2>1{ //if statement
        println!("{}","first");
    }else{ //else statement
        println!("{}","second");
    }*/
    let stt:&str = "hello";
    //println!("{}",stt.len()) //lenght of an string
}
