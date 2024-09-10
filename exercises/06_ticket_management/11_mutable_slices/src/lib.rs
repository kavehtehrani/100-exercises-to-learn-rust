// TODO: Define a function named `lowercase` that converts all characters in a string to lowercase,
//  modifying the input in place.
//  Does it need to take a `&mut String`? Does a `&mut [str]` work? Why or why not?

// this took quite a while to do. the posted solution is to a different problem at
// https://github.com/mainmatter/100-exercises-to-learn-rust/blob/solutions/exercises/06_ticket_management/11_mutable_slices/src/lib.rs
// &mut str works because you can change the content of the string in place, which is effectively a vector of chars.
// I used unsafe since that's what the make_ascii_lowercase() uses.
pub fn lowercase(s: &mut str) {
    for i in 0..s.len() {
        let bytes = unsafe { s.as_bytes_mut() };
        for byte in bytes {
            *byte = byte.to_ascii_lowercase();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut s = String::from("");
        lowercase(&mut s);
        assert_eq!(s, "");
    }

    #[test]
    fn one_char() {
        let mut s = String::from("A");
        lowercase(&mut s);
        assert_eq!(s, "a");
    }

    #[test]
    fn multiple_chars() {
        let mut s = String::from("Hello, World!");
        lowercase(&mut s);
        assert_eq!(s, "hello, world!");
    }

    #[test]
    fn mut_slice() {
        let mut s = "Hello, World!".to_string();
        lowercase(s.as_mut_str());
        assert_eq!(s, "hello, world!");
    }
}
