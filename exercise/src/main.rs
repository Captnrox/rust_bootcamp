/* 
*  main()
* Learning Rust!
*/
fn main() {
    let mut counter = 5;
    let mut num = 0;
    while counter > 0{
        num = (num + 1) * 2;
        counter-= 1;
    } 
     //declare variable
    let value = if num < 25 {num/2} else {num*2};

    println!("value: {}",value);


    let a:[u8; 5] = [10,22,37,40,50];
    let result = weird_loop(a);

    println!("Result: {}", result);

}

fn weird_loop(a:[u8; 5]) -> u8 {


    'outer: for elem in a {
        'inner: for i in 0..5{
            if elem == i*10 + 7{
                break 'outer;
            }
            if elem == i*10 + 2{
                break 'inner;
            }

            println!("{} != {} <- {} ", elem, i*10+7,i);
        }
    } 

    a[4]
} 
