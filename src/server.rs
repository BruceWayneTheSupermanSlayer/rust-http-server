use std::net::TcpListener;
use std::io::Read;

pub struct Server {
  address: String,
}

impl Server {
  /**
   * `new` is kind of a static function, the way to know this is that we are not passing `self` as first argument
   * new is kind of accepted as constructor in Rust but this is just common practise
   */
  pub fn new(address: String) -> Self {
    Self { address }
  }

  /**
   * `run` would take ownership of entire struct in our case server, as we want to keep our server running this is fine
   * If we do not want this kind of behaviour then
   * we can either pass a refrence or a mutuable refrence
   * run(&mut self)
   *
   * `run` is kind of a instance variable and the way we know this is we have `self` as the first parameter
   */
  pub fn run(self) {
    println!("server running on {}", self.address);
    let tcp_listenr = TcpListener::bind(&self.address).unwrap();

    // rust has an built in syntax for infinite loop
    // in rust loops can have lables or kind of name
    // the way to do this is to add `'` followed by name
    'infinite_loop: loop {
      match tcp_listenr.accept() {
        Ok((mut stream, _)) => {
          println!("TCP connection established successfully");
          // [0; 1024] -> this creates an array of length 1024 and fills all values with 0
          let mut buffer = [0; 1024];
          match stream.read(&mut buffer) {
            Ok(_) => {
              println!("Recevied Request {} ", String::from_utf8_lossy(&buffer))
            }
            Err(e) => println!("failed to read connection {}", e)
          }
        }
        Err(e) => println!("Failed to eastablish TCP Connection! {}", e)
      }
    }
  }
}

