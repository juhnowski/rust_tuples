fn main() {
    let tup: (i32, f64, bool) = (1,1.5,true);
    let (x,y,z) = tup;
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    println!("x={} y={} and z={}",x,y,z);
    println!("tup.0={} tup.1={} and tup.2={}",a,b,c);
}
