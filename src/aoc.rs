macro_rules! advent_of_code(($($id:ident -> $sol:pat)+) => {
  $(mod $id;)+

  fn main() {
    // open files
    $(let $id = {
      let path = concat!("input/", stringify!($id));
      std::fs::read_to_string(path).unwrap()
    };)+

    // spawn and join threads
    let t = std::time::Instant::now();
    $(let $id = std::thread::spawn(move || $id::run(&$id));)+
    $(let $id = $id.join();)+
    println!("{:?}", t.elapsed());

    // check solutions
    $(match $id {
      Ok($sol) | Err(_) => {},
      Ok(res) => println!(
        concat!(stringify!($id), " returned {:?}, expected ", stringify!($sol)),
        res
      )
    })+
  }
});
