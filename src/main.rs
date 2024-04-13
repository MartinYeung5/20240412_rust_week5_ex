macro_rules! multiply_number_from_one{
    (
        $($a:expr),*
    )
    =>
    {
       { 1  $(*$a)* }
    }
}

macro_rules! add_as{
    (
        $a:expr,$b:expr,$typ:ty
    )=>
    {
        $a as $typ + $b as $typ
    }
}

fn main(){
    println!("{}",multiply_number_from_one!(1,2,3,4));
    println!("{}",add_as!(0,2,u8));
}