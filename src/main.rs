mod immutable;
mod mergesort;
mod higher_order;

fn main() {
    println!("How functional is Rust? Research prototypes\n");
    immutable::demo();
    mergesort::demo();
    higher_order::demo();
}
