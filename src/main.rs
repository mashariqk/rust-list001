use mylists::first::List;

fn main(){

    let mut myfl = List::new();
    myfl.push(23);
    myfl.push(37);
    myfl.push(71);
    println!("{:?} and length is {}",myfl,myfl.len());
    println!("{:?}",myfl.pop());
    println!("{:?}",myfl);
}