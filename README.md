# avltree
A rust implementation of AVL Tree. 

I start this project for a better understanding of the unsafe part of rust,
so I use `Option<NonNull<Node>>` instead of `Option<Box<Node>>`.
As a result, there are a lot of unsafe blocks whenever it needs to access the child or parent.

It's still under construction.

## Usage
```rust
use avltree::AVLTree;

let mut avl = AVLTree::<i32, i32>::new();
avl.insert(1, 42);
assert_eq!(avl.get(&1), Some(&42));
avl.delete(&1);
```

## Performance
It's slightly faster than some other rust alternatives I found on github.
However, it's nearly four times slower than std::collections::BTreeMap 
and the C version AVL tree implemantation [avlmini](https://github.com/skywind3000/avlmini).


On my computer, inserting 10,000,000 random keys.

avltree: ~15s

some-other-avltree-A: ~18s

some-other-avltree-B: ~16s

BTreeMap|avlmini|linus_rbtree: ~4s