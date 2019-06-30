use mylists::first::List;

fn main(){

    let mut myfl = List::new();
    myfl.push(23);
    myfl.push(37);
    myfl.push(71);
    println!("{:?}",myfl);
    println!("{:?}",myfl.pop());
    println!("{:?}",myfl);
}