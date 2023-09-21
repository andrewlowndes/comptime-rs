fn main() {
    println!(
        "The following is ran at compile time: {}",
        comptime::comptime! {
            vec![1,2,3].iter().sum::<u32>()
        },
    );
}
