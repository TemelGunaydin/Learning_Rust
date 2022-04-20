/*
    Http Server Implementation using Rust
*/ 
fn main() {
    /*
    This section only describes the differences between String and String slices.
    let string = String::from("127.0.0.1:8080"); //We use string slice &str here. String slice is just a view inside a string that is mutable.
    let string_slice = &string[10..];//using indeksing in string slices is not efficient. It will give the value of starting from 10th byte. If the character located in the 10th byte can occupy more than 1 byte and string[10..] can give different output than we expected.
    
    dbg!(&string); //dbg macro doesnt take ownership of string. If it does, we cant use string variable here. Because on line 6 we borrowed the value to string_slice.So this line doesnt give error 
    
    dbg!(string_slice);
    let string_borrow :str= &string;
    */
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
struct Server { //We will use struct and will have one property addr as String
   addr : String,
}
impl Server { //We will implement a function and a method for Struct Server
    fn new(addr:String) ->  Self { //new is a method and returns a Server instance. Every struct has a type called Self and it can be used as an alias for struct name.
        Self //This can be written as Server. Because on line 9 we return a Server instance and we called it as server. 
        {
            addr //if the name of the parameter in new function is same with the name defined in Struct implementation(addr in this case) we can only use the parameter name .
        }     
    }

    fn run(self) { //Owns the struct Server. run is used by using server instance on line 14. And if we use self here run owns the Server instance. And after run function we cant use Server instance. Because we owned the variable server.
        println!("Listening on {} ",self.addr);
    }
}

