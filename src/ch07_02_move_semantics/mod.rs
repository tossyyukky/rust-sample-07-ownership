
use super::ch07_01_value_scop::Parent;
use super::ch07_01_value_scop::Child;

pub fn main() {
    let mut p1 = Parent(1, Child(11), Child(12));

    let p2 = p1;

    println!("p2: {:?}", p2);
    // println!("p1: {:?}", p1); // p1は値の所有権を失ったのでアクセス不可
    // -> error[E0382]: borrow of moved value: p1

    p1 = Parent(2, Child(21), Child(22));
    println!("p1: {:?}", p1); // p1は別の値の所有権を持つのでアクセスできる
}