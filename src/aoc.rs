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
    $(let $id = std::thread::spawn(move || {
      let t = std::time::Instant::now();
      ($id::run(&$id), t.elapsed())
    });)+
    $(let $id = $id.join();)+
    let t = t.elapsed();

    // check solutions
    $(match $id {
      Err(_) => {}
      Ok(($sol, t)) => println!(
        concat!(stringify!($id), "{: >9}μs"),
        t.as_micros()
      ),
      Ok((res, _)) => println!(
        concat!(stringify!($id), " returned {:?}, expected ", stringify!($sol)),
        res
      )
    })+
    println!("all{: >11}μs", t.as_micros());
  }
});
