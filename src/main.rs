fn main() {
    string_training()
}

pub fn ownership_range(){
    {                    // s n'est pas en vigueur ici, elle n'est pas encore déclarée
            let s = "hello"; // s est en vigueur à partir de ce point
            println!("{}",s)
            // on fait des choses avec s ici
    }                    // cette portée est maintenant terminée, et s n'est plus en vigueur
}

pub fn string_training(){

    let mut s1 = String::from("hello");
    s1.push_str(" world!");
    let s2 = s1.clone();
    println!("{}, {}",s1,s2)

}