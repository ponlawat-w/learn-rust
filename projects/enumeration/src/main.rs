enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String)
}
impl IpAddress {
    fn print_address(self: &Self) {
        match self {
            Self::V4(i0, i1, i2, i3) => {
                println!("{}.{}.{}.{}", i0, i1, i2, i3);
            }, Self::V6(addr) => {
                println!("ipv6({})", addr);
            }
        }
    }
}

enum Coin {
    Penny, Nickel, Dime, Quarter
}
impl Coin {
    fn value(self: &Self) -> u8 {
        match self {
            Self::Penny => 1,
            Self::Nickel => 5,
            Self::Dime => 10,
            Self::Quarter => 25,
        }
    }
}

fn main() {
    {
        let home = IpAddress::V4(127, 0, 0, 1);
        let loopback = IpAddress::V6(String::from("::1"));

        home.print_address();
        loopback.print_address();
    } {
        let some_number = Some(5);
        let some_char = Some('e');
        let absent_number: Option<i32> = None;

        let six = match some_number {
            Some(x) => x + 1,
            None => 0
        };
        let mut msg = String::from("My letter is: ");
        msg.push(match some_char {
            Some(x) => x,
            None => ' '
        });
        let result = match absent_number {
            Some(x) => x * 2,
            None => 0
        };

        println!("six = {}", six);
        println!("msg = {}", msg);
        println!("result = {}", result);
    } {
        let coin1 = Coin::Penny;
        let coin2 = Coin::Nickel;
        let coin3 = Coin::Dime;
        let coin4 = Coin::Quarter;
        println!("Coin 1 has value {}", coin1.value());
        println!("Coin 2 has value {}", coin2.value());
        println!("Coin 3 has value {}", coin3.value());
        println!("Coin 4 has value {}", coin4.value());
    } {
        let num = 3;
        let x = match num {
            1 => 0,
            2 => 0,
            x => x * 2,
        };
        println!("x = {}", x);
    } {
        let num = 5;
        match num {
            1 => { println!("one"); },
            2 => { println!("two"); },
            3 => { println!("three"); },
            _ => { println!("other"); }
        }
    } {
        let some_number = Some(3);
        if let Some(num) = some_number {
            println!("num = {}", num);
        } else {
            println!("None");
        }
    } {
        let some_number: Option<i32> = None;
        if let Some(num) = some_number {
            println!("num = {}", num);
        } else {
            println!("None");
        }
    }
}
