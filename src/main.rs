fn ordinal_word(number:i32)->&'static str{
    match number {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "unknown",
    }
}

fn gift_of_day(number:i32)->&'static str{
    match number {
        1=>"partridge in a pear tree!",
        2=>"Two turtle doves,",
        3=>"Three French hens,",
        4=>"Four calling birds,",
        5=>"Five gold rings,",
        6=>"Six geese a-laying,",
        7=>"Seven swans a-swimming,",
        8=>"Eight maids a-milking,",
        9=>"Nine ladies dancing,",
        10=>"Ten lords a-leaping,",
        11=>"Eleven pipers piping,",
        12=>"Twelve drummers drumming,",
        _=>"unknown",
    }
}

fn main() {
    for num in 1..13 {
        let ord = ordinal_word(num);
        println!("On the {ord} day of Christmas,\nmy true love sent to me");
        for gift_num in (1..num+1).rev() {
            let gift = gift_of_day(gift_num);
            if gift_num == 1 {
                if num == 1 {
                    println!("A {gift}")
                } else {
                    println!("And a {gift}")
                }
            }
            else {
                println!("{gift}")
            }
        }
        println!()
    }
}

/*
On the twelfth day of Christmas,
my true love sent to me
Twelve drummers drumming,
Eleven pipers piping,
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five gold rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree!
*/