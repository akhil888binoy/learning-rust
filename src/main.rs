fn main() {
    println!("Hello, world!");
    let x:i32 = -4;
    let y:u32 = 5;
    let z:f32 = 5.3;
    print!("{} {} {}", x, y, z);

    let isMale: bool = true ;
    let isFemale : bool = false;
    if isMale{
        println!("it is male");
    }
    else if isFemale{
        println!("is female");
    }
    let greetins : String = String :: from ("hello");
    println!("{}",greetins);

    for i  in  0..11{
        print!("{}",i);
    }

    let sentence: String = String :: from("Hello World");
    let firstWord = getfirstword(sentence);
    print!("{}" , firstWord);

    


}


fn getfirstword(sentence : String ) -> String {
    let mut ans:String = String :: from("");
    for char in sentence.chars(){
        ans.push_str( char.to_string().as_str());
        if char == ' '{
            break
        }
    }
    return ans;
}

