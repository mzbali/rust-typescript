use std::collections::HashMap;

fn main() {
    let data = vec![1, 2, 3];
    let mut foo = data.iter().map(|x| x + 1);
    let mut new_vec = vec![];
    // equivalent to collect()
    while let Some(x) = foo.next() {
        new_vec.push(x);
    }
    println!("{:?}", new_vec);

    let how_many_times: usize = vec![1, 2, 3].into_iter().skip(2).count();
    println!("{:?}", how_many_times);

    vec![1, 2, 5, 9, 4]
        .iter()
        .skip(2)
        .take_while(|&&x| x > 4)
        .for_each(|x| println!("{}", x));

    let hash_map = HashMap::from([("foo", 1), ("bar", 2), ("roo", 3)]);

    hash_map.iter().for_each(|(k,v)| println!("{}:{}", k, v));  
}
