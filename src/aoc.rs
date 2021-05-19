macro_rules! advent_of_code(($($id:ident -> $sol:pat)+) => {
  $(mod $id;)+

  fn main() {
    let t = std::time::Instant::now();

    // open files
    $(let $id = {
      let path = concat!("input/", stringify!($id));
      std::fs::File::open(path).unwrap()
    };)+

    // spawn and join threads
    $(let $id = std::thread::spawn(move || $id::run($id));)+
    $(let $id = $id.join();)+

    // check solutions
    $(match $id {
      Ok($sol) | Err(_) => {},
      Ok(res) => println!(
        concat!(stringify!($id), " returned {:?}, expected ", stringify!($sol)),
        res
      )
    })+

    println!("{:?}", t.elapsed());
  }
});
