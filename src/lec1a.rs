fn main (){
    println!("{}",abs(-10));
}

fn abs(x:i32)->i32{
    if x<0 {
        return -x;
    }else{
        return x;
    }
}