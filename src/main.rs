mod ch07_01_value_scop;
mod ch07_02_move_semantics;
mod ch07_03_nll;

fn main() {
    println!("Hello, world!");
    ch07_01_value_scop::ch_main();
    ch07_02_move_semantics::main();
    ch07_03_nll::main();
}
