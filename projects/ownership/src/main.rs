fn main() {
    {
        let s1 = String::from("Hello!");
        let s2 = s1;
    
        println!("{}", s2);
        // println!("{}", s1);
    } {
        let s1 = String::from("Hello!");
        let s2 = s1.clone();
        println!("{}", s1);
        println!("{}", s2);
    } {
        let s1 = String::from("Hello World!");
        print_string(s1);

        let x: i32 = 5;
        print_integer(x);
        
        // println!("{}", s1);
        println!("{}", x);
    } {
        let s1 = String::from("Hello World!");
        let s2 = print_and_return(s1);
        // println!("{}", s1);
        println!("{}", s2);
        
        let mut s3 = String::from("Hello World!");
        s3 = print_and_return(s3);
        println!("{}", s3);
    } {
        let s1 = String::from("Hello World!");
        print_reference(&s1);
    } {
        let s1 = String::from("Hello World!");
        let size = get_string_length(&s1);
        println!("{}", size);
    } {
        let mut s1 = String::from("Hello world");
        alter_string(&mut s1);
        println!("{}", s1);
    } {
        let mut str = String::from("Hello world");
        let ms1 = &mut str;
        // let ms2 = &mut str;
        alter_string(ms1);
        // alter_string(ms2);
        println!("{}", ms1);
        // println!("{}", ms2);
    } {
        let mut str = String::from("Hello world");
        {
            let ms1 = &mut str;
            alter_string(ms1);
            println!("{}", ms1);
        } {
            let ms2 = &mut str;
            alter_string(ms2);
            println!("{}", ms2);
        }
    } {
        let mut str = String::from("Hello World!");
        let ims1 = &str;
        let ims2 = &str;
        println!("-> {}\n-> {}", ims1, ims2);
        let ms1 = &mut str;
        ms1.push_str("!!!");
        println!("{}", str);
    } {
        let str = String::from("Hello World");
        let hello = &str[0..5];
        let world = &str[6..11];
        println!("{}_{}", hello, world);
    } {
        let mut message = String::from("Hello World!");
        println!("Messaage: {}", message);
        let word = first_word_idx(&message);
        println!("First word end index: {}", word);
        
        message.clear();
        println!("Messaage: {}", message);
    } {
        let message = String::from("Hello World!");
        let hello = &message[.."hello".len()];
        let world = &message[6..];
        let full_message = &message[..];
        println!("1: {}, 2: {}, 3: {}", hello, world, full_message);
    } {
        let mut message = String::from("Hello World!");
        let word = get_first_word(&message);
        println!("message: {}", message);
        // message.clear();
        println!("word: {}", word);
        message.clear();
        println!("message: {}", message);
    } {
        let mut message = String::from("Hello World!");
        let word = get_new_first_word(&message);
        println!("message: {}", message);
        message.clear();
        println!("word: {}", word);
    }
}

fn print_string(message: String) {
    println!("{}", message);
}

fn print_and_return(message: String) -> String {
    println!("{}", message);
    return message;
}

fn print_reference(message: &String) {
    println!("{}", message);
}

fn get_string_length(message: &String) -> usize {
    return message.len();
}

fn print_integer(i: i32) {
    println!("{}", i);
}

// fn alter_string(str: &String) {
//     str.push_str(".");
// }
fn alter_string(str: &mut String) {
    str.push('.');
}

fn first_word_idx(str: &String) -> usize {
    let bytes = str.as_bytes();

    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return index;
        }
    }

    str.len()
}

fn get_first_word(str: &str) -> &str {
    let bytes = str.as_bytes();

    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &str[..index];
        }
    }

    &str[..]
}

fn get_new_first_word(str: &str) -> String {
    String::from(get_first_word(str))
}
