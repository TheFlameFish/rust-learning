use text_io::read;

fn main() {
    print!("Please enter 𝑛 of 𝐹 that you would like to find. 𝑛 must be a positive integer. \n > ");
    let n: u128 = read!();

    if n > 185 {
        panic!("Sorry! You can't go past F_185. (It overflows using unsigned 128 bit integers)");
    }

    let mut previous: u128 = 0;
    let mut next_previous: u128 = 1;

    for _ in 0..n {
        let new = previous + next_previous;

        next_previous = previous;
        previous = new;
    }

    println!("F_{n} = {previous}");
}
