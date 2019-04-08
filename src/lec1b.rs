fn sos(x:i32,y:i32)->i32{
    let sq = |x:i32|->i32{x*x};
    return sq(x)+sq(y);
}


fn oneplus(x:i32)->i32{
    x+1
}
fn add(x:i32,y:i32)->i32{
    if x==0{
        y
    }
    else{
        add(x-1,oneplus(y))
    }
}


fn fib(n:i32)->i32{
    if n < 2{
        return n;
    }else{
        return fib(n-1)+fib(n-2);
    }
}


// fn sqrt(x:i32)->i32{

//     let tolerance = 0.00001;
    
//     fn f(start:i32,x:i32)->i32{
//         return (start+x/start)/2;
//     }

//     fn find_fix_point(f:fn(i32,i32)->i32,start:i32)->i32{
//         fn iter(old:i32,new:i32)->i32{
//             if (old-new).abs()<tolerance{
//                 return new
//             }else{
//                 return iter(new,f(new,x))
//             }
//         }
//         return iter(start,f(start,x))
//     }


//     return find_fix_point(f,1);
// }


fn main(){
    println!("{}",add(3,4));
    println!("{}",sos(3,4));
    println!("{}",fib(10));
}