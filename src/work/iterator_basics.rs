fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    for num in numbers.iter() {
        println!("val: {}", num);
    }


    let mut iter = numbers.iter();
    println!("{:?}", iter.next()); 
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());


    let mut scores = vec![10, 20, 30];
    for score in scores.iter_mut() {
        *score += 5;
    }
    println!("updated scores: {:?}", scores);

 

    let evens_doubled: Vec<i32> = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * 2)
        .collect();

    println!("evens doubled: {:?}", evens_doubled);

    let total: i32 = numbers.iter().sum();
    println!("sum of numbers: {}", total);
}
