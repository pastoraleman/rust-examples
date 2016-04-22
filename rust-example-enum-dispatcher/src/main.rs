// Dispatcher code example using enums

enum Dispatcher {
    Twitter,
    Syndication,
    Syslog,
    Console,
    Email
}

// We have a message, and then a list of desinations that the message will be dispached to.
struct MyMessage {
    message: String,
    outputs: Vec<Dispatcher>
}


fn main() {

    let mut new_msg = MyMessage {
        message: "This is my message!".to_string(),
        outputs: vec![]
    };

    // Add in a list of message dispatch targets
    new_msg.outputs.push(Dispatcher::Syndication);
    new_msg.outputs.push(Dispatcher::Twitter);
    new_msg.outputs.push(Dispatcher::Console);
    new_msg.outputs.push(Dispatcher::Email);


    for output in new_msg.outputs {
        match output {
            Dispatcher::Twitter => println!("+ Sent to Twitter channel! > {}", new_msg.message),
            Dispatcher::Syndication => println!("+ Sent to Atom/RSS feed! > {}", new_msg.message),
            Dispatcher::Syslog => println!("+ Sent to syslog server! > {}", new_msg.message),
            Dispatcher::Console => println!("{}", new_msg.message),
            _ => println!("- Message not dispatched to unsupported target.")
        }
    }


}
