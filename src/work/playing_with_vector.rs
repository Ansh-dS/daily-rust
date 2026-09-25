

fn main(){
    let mut dummy= Vec::new();

    for val in 0..10 { 
        dummy.push(val);
    } 

    let evens = get_positive_values(&dummy);
    println!("{:?}", evens);
}

fn get_positive_values(vec: &Vec<u8>) -> Vec<u8> {
    let mut ans = Vec::new(); 
    for val in vec {
        if val % 2 == 0 {
            ans.push(*val); 
        }
    }
    ans
}