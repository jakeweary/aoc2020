use std::io::{BufRead, BufReader, Read};

pub fn line_groups<R: Read>(r: R) -> impl Iterator<Item = String> {
  let mut rdr = BufReader::new(r);
  let mut buf = Vec::new();

  std::iter::from_fn(move || {
    buf.clear();

    loop {
      match (rdr.read_until(b'\n', &mut buf).ok()?, &buf[..]) {
        (0, []) => return None,
        (0, _) => break,
        (1, [b'\n']) => { buf.pop(); }
        (1, [.., b'\n']) => { buf.pop(); break; }
        _ => {}
      }
    }

    String::from_utf8(buf.clone()).ok()
  })
}

#[cfg(test)]
mod tests {
  fn lg(text: &'static str) -> impl Iterator<Item = String> {
    let cur = std::io::Cursor::new(text.as_bytes());
    super::line_groups(cur)
  }

  #[test]
  fn normal_input() {
    assert!(lg("").eq(None::<&str>));
    assert!(lg("\n").eq(None::<&str>));
    assert!(lg("#\n").eq(["#\n"]));
    assert!(lg("#\n#\n").eq(["#\n#\n"]));
    assert!(lg("#\n#\n\n#\n#\n").eq(["#\n#\n", "#\n#\n"]));
  }

  #[test]
  fn edge_cases() {
    assert!(lg("#").eq(["#"]));
    assert!(lg("\n#").eq(["#"]));
    assert!(lg("\n\n").eq(None::<&str>));
    assert!(lg("\n\n\n").eq(None::<&str>));
    assert!(lg("\n\n\n\n").eq(None::<&str>));
    assert!(lg("#\n#\n\n#\n#").eq(["#\n#\n", "#\n#"]));
    assert!(lg("#\n#\n\n#\n#\n\n").eq(["#\n#\n", "#\n#\n"]));
    assert!(lg("#\n#\n\n\n#\n#\n\n\n").eq(["#\n#\n", "#\n#\n"]));
  }
}
