fn main() {
    let s = String::from("hello world");
    let idx = first_world(&s);
    let s_slice = first_world_slice(&s);
    println!("{}", idx);
    println!("{}", s_slice);
 }
 
 fn first_world(s : &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
       if item == b' ' {
          return i;
       }
    }
 
    s.len()
 }

 fn first_world_slice(s :&str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[.. i]
        }
    }

    return &s[..]
 } 