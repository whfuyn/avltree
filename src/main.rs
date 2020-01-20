use avltree::avl::AVLTree;

fn main(){
    let mut avl = AVLTree::<i32, i32>::new();
    let mut h = 0;
    let mut cnt = 0;
    use rand;
    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    use std::time::Instant;

    let mut nums: Vec<i32> = (0..10_000_000).collect();
    nums.shuffle(&mut rng);
    let mut now = Instant::now();
    for i in nums.iter() {
        avl.insert(*i, -i);
        // cnt += 1;
        // if i % 13 == 7 {
        //     avl.delete(&(i - 2));
        //     cnt -= 1;
        //     assert_eq!(avl.get(&(i - 2)), None);
        // }
        // let nh = avl.get_height();
        // if h != nh {
            // h = nh;
        //     println!("i: {}  cnt: {}  height: {}", i, cnt, h);
        // }
    }
    let t1 = now.elapsed().as_millis();
    h = avl.get_height();
    now = Instant::now();
    let mut map = std::collections::BTreeMap::<i32, i32>::new();
    for i in nums.iter() {
        map.insert(*i, -i);
    }
    let t2 = now.elapsed().as_millis();
    println!("height: {}  avl: {}ms  btreemap: {}ms", h, t1, t2);
    // println!("cnt: {}  height: {}", cnt, h);
}
