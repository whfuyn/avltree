use std::cmp::Ordering;
use std::ptr::NonNull;
use std::{fmt, mem};
use std::fmt::Display;
use std::fmt::Debug;

#[derive(Debug)]
struct Node<K: Ord, V> {
    key: K,
    value: V,
    height: usize,
    pa: Option<NonNull<Node<K, V>>>,
    lch: Option<NonNull<Node<K, V>>>,
    rch: Option<NonNull<Node<K, V>>>,
}

impl<K: Ord + Debug, V: Debug> Node<K, V> {
    fn new(key: K, val: V) -> Self {
        Self {
            key: key,
            value: val,
            height: 0,
            pa: None,
            lch: None,
            rch: None,
        }
    }

    fn update_height(&mut self) {
        unsafe {
            let get_height =
                |ch: Option<NonNull<Self>>| ch.map_or(0, |pnode| pnode.as_ref().height);
            let lh = get_height(self.lch);
            let rh = get_height(self.rch);
            self.height = std::cmp::max(lh, rh) + 1;
            // if std::cmp::max(lh, rh) - std::cmp::min(lh, rh) == 3 {
            //     dbg!(&self);
            //     dbg!(self.lch.unwrap().as_ref());
            //     dbg!(self.rch.unwrap().as_ref());
            //     self.travel();
            //     panic!();
            // }
        }
    }

    fn calc_balance_factor(&self) -> isize {
        unsafe {
            let get_height =
                |ch: Option<NonNull<Self>>| ch.map_or(0, |pnode| pnode.as_ref().height);
            let lh = get_height(self.lch);
            let rh = get_height(self.rch);
            if lh >= rh {
                (lh - rh) as isize
            } else {
                -((rh - lh) as isize)
            }
        }
    }

    fn fixup(&mut self) -> Option<NonNull<Self>> {
        unsafe {
            let mut x = NonNull::<Self>::new_unchecked(self as *mut _);
            let old_height = x.as_ref().height;
            x.as_mut().update_height();
            let x_factor = x.as_ref().calc_balance_factor();
            match x_factor {
                -1..=1 => (),
                -2 | 2 => {
                    let mut ch = if x_factor == -2 {
                        x.as_mut().rch
                    }
                    else {
                        x.as_mut().lch
                    }.unwrap();
                    let ch_factor = ch.as_ref().calc_balance_factor();
                    if x_factor.signum() * ch_factor.signum() == -1 {
                        match ch_factor {
                            -1 => ch.as_mut().rotate_left(),
                            1 => ch.as_mut().rotate_right(),
                            bad_factor => panic!(format!(
                                "Corrupted node: `{} - {}`.",
                                x_factor,
                                bad_factor
                            )),
                        }
                    }
                    if x_factor == 2 {
                        x.as_mut().rotate_right();
                    }
                    else {
                        x.as_mut().rotate_left();
                    }
                    x = x.as_ref().pa.unwrap();
                }
                _ => {
                    panic!(format!(
                        "AVL node corrupted with balance_factor {}.",
                        x_factor
                    ));

                }
            }
            let new_height = x.as_ref().height;
            if let Some(mut pa) = x.as_ref().pa {
                if new_height != old_height {
                    pa.as_mut().fixup()
                } else {
                    None
                }
            } else {
                Some(x)
            }
        }
    }

    fn rotate_right(&mut self) {
        assert!(self.lch.is_some());
        unsafe {
            let opa = self.pa;
            let mut lch = self.lch.unwrap();
            let mut x = NonNull::new_unchecked(self as *mut _);
            let olrch = mem::replace(&mut lch.as_mut().rch, Some(x));
            self.lch = olrch;
            self.pa = Some(lch);
            if let Some(mut lrch) = olrch {
                lrch.as_mut().pa = Some(x);
            }
            if let Some(mut pa) = opa {
                let pa_lch = pa.as_ref().lch;
                if Some(x) == pa_lch {
                    pa.as_mut().lch = Some(lch);
                } else {
                    pa.as_mut().rch = Some(lch);
                }
            }
            lch.as_mut().pa = opa;
            x.as_mut().update_height();
            lch.as_mut().update_height();
            // if let Some(mut pa) = opa {
            //     pa.as_mut().update_height();
            // }
        }
    }

    fn rotate_left(&mut self) {
        assert!(self.rch.is_some());
        unsafe {
            let opa = self.pa;
            let mut rch = self.rch.unwrap();
            let mut x = NonNull::new_unchecked(self as *mut _);
            let orlch = mem::replace(&mut rch.as_mut().lch, Some(x));
            self.rch = orlch;
            self.pa = Some(rch);
            if let Some(mut rlch) = orlch {
                rlch.as_mut().pa = Some(x);
            }
            if let Some(mut pa) = opa {
                let pa_lch = pa.as_ref().lch;
                if Some(x) == pa_lch {
                    pa.as_mut().lch = Some(rch);
                } else {
                    pa.as_mut().rch = Some(rch);
                }
            }
            rch.as_mut().pa = opa;
            x.as_mut().update_height();
            rch.as_mut().update_height();
            // if let Some(mut pa) = opa {
            //     pa.as_mut().update_height();
            // }
        }
    }
}

impl<K: Ord + Debug, V: Debug> Node<K, V> {
    fn travel(&self) {
        let print = || {
            print!(
                "p: {:?}\nkey: {:?}\nheight: {}\nfactor: {}\nlch: {:?}  rch: {:?}\n",
                self as * const _,
                self.key,
                self.height,
                self.calc_balance_factor(),
                self.lch,
                self.rch,
            )
        };
        unsafe {
            let travel = |pch: NonNull<Node<K, V>>| pch.as_ref().travel();
            self.lch.map(travel);
            print();
            self.rch.map(travel);
        }
    }
}

#[derive(Debug)]
pub struct AVLTree<K: Ord + Debug, V: Debug> {
    root: Option<NonNull<Node<K, V>>>,
}

impl<K: Ord + Debug, V: Debug> AVLTree<K, V> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: K, val: V) {
        unsafe {
            let boxed = Box::new(Node::new(key, val));
            let mut new = Box::into_raw_non_null(boxed);
            let mut pp = &mut self.root as *mut Option<NonNull<Node<K, V>>>;
            while let Some(mut p) = *pp {
                match new.as_ref().key.cmp(&p.as_ref().key) {
                    Ordering::Less | Ordering::Equal => {
                        pp = &mut p.as_mut().lch as *mut _;
                    }
                    Ordering::Greater => {
                        pp = &mut p.as_mut().rch as *mut _;
                    }
                }
                new.as_mut().pa = Some(p);
            }
            *pp = Some(new);
            self.root = new.as_mut().fixup().or(self.root);
        }
    }

    pub fn delete(&mut self, key: &K) {
        let x = self.search(key);
        if x.is_none() {
            return;
        }
        let mut x = x.unwrap();
        let delete_simple = |mut x: NonNull<Node<K, V>>| unsafe {
            let opa = x.as_mut().pa;
            let new_root = if let Some(mut pa) = opa {
                let pa_lch = pa.as_ref().lch;
                let och = x.as_ref().lch.or(x.as_ref().rch);
                if pa_lch == Some(x) {
                    pa.as_mut().lch = och;
                } else {
                    pa.as_mut().rch = och;
                }
                if let Some(mut ch) = och {
                    ch.as_mut().pa = Some(pa);
                }
                pa.as_mut().fixup()
            } else {
                x.as_ref().lch.or(x.as_ref().rch)
            };
            Box::from_raw(x.as_ptr());
            new_root
        };
        if self.get_height() == 1 {
            delete_simple(x);
            self.root = None;
            return;
        }
        unsafe {
            self.root = match x.as_ref() {
                Node { lch: None, .. } | Node { rch: None, .. } => delete_simple(x),
                _ => {
                    let mut p = x.as_ref().rch.unwrap();
                    while let Some(q) = p.as_ref().lch {
                        p = q;
                    }
                    mem::swap(&mut x.as_mut().key, &mut p.as_mut().key);
                    mem::swap(&mut x.as_mut().value, &mut p.as_mut().value);
                    delete_simple(p)
                }
            }
            .or(self.root);
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        unsafe { self.search(key).map(|pn| &(*pn.as_ptr()).value) }
    }

    pub fn get_height(&self) -> usize {
        unsafe { self.root.map(|p| p.as_ref().height).unwrap_or(0) }
    }

    fn search(&self, key: &K) -> Option<NonNull<Node<K, V>>> {
        let mut opn = self.root;
        unsafe {
            while let Some(pn) = opn {
                match key.cmp(&pn.as_ref().key) {
                    Ordering::Less => opn = pn.as_ref().lch,
                    Ordering::Equal => return Some(pn),
                    Ordering::Greater => opn = pn.as_ref().rch,
                }
            }
        }
        None
    }
}

impl<K: Ord + Debug, V: Debug> AVLTree<K, V> {
    pub fn travel(&self) {
        unsafe {
            self.root.map(|pn| pn.as_ref().travel());
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert() {
        let mut avl = AVLTree::<i32, i32>::new();
        avl.insert(1, 101);
        avl.insert(3, 103);
        avl.insert(5, 105);
        assert_eq!(avl.get(&1), Some(&101));
        assert_eq!(avl.get(&2), None);
        assert_eq!(avl.get(&3), Some(&103));
        assert_eq!(avl.get(&4), None);
        assert_eq!(avl.get(&5), Some(&105));
        avl.travel();
    }
    #[test]
    fn test_delete() {
        // {
        //     let mut avl = AVLTree::<i32, i32>::new();
        //     avl.insert(1, 101);
        //     avl.delete(&1);
        //     assert_eq!(avl.get(&1), None);
        //     avl.insert(1, 101);
        //     avl.insert(2, 102);
        //     avl.delete(&1);
        //     assert_eq!(avl.get(&1), None);
        //     assert_eq!(avl.get(&2), Some(&102));
        // }
        // {
        //     let mut avl = AVLTree::<i32, i32>::new();
        //     avl.insert(2, 102);
        //     avl.insert(1, 101);
        //     avl.delete(&2);
        //     assert_eq!(avl.get(&2), None);
        //     assert_eq!(avl.get(&1), Some(&101));
        // }
        {
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
            panic!("height: {}  avl: {}ms  btreemap: {}ms", h, t1, t2);
            // println!("cnt: {}  height: {}", cnt, h);
        }
    }
}
