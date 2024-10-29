fn main() {
    const NUMBERS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    const GIFTS: [&str; 12] = [
        "\nA partridge in a pear tree",
        "\nTwo turtle doves and",
        "\nThree French hens",
        "\nFour calling birds",
        "\nFive golden rings",
        "\nSix geese a-laying",
        "\nSeven swans a-swimming",
        "\nEight maids a-milking",
        "\nNine ladies dancing",
        "\nTen lords a-leaping",
        "\nEleven pipers piping",
        "\nTwelve drummers drumming",
    ];

    for i in 0..12 { // Do keep in mind the fact that this iterates through 0-11 inclusive.
        let mut gifts: Vec<&str> = GIFTS[0..=i].to_vec(); // I think =i is about equivalent to saying i+1
        gifts.reverse();

        let gifts: String = gifts.concat();

        println!("On the {} day of christmas my true lover gave to me {gifts}.\n",NUMBERS[i]);
    }
}
