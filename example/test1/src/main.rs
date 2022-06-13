//use std::thread::available_parallelism;

fn main() {
    let a = [1,2,3,4,5,6,10];
    let b=[3,2,10];
    let m= a.len();
    let n = b.len();
    for i in 0..m{
        if a[i]==b[0] && m+1-i>n{
            println!("{}",i);
            for j in 0..n{
                if a[i+j]!=b[j]{
                    println!("{}",j);
                    println!("Mảng nhỏ không phải là mảng con của mảng lớn!");
                    break;
                }else if j==n-1{
                    println!("Mảng nhỏ là mảng con của mảng lớn!");
                }
            }
        }
    }
}