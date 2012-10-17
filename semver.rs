use std;
import io::{reader, reader_util};

export version;
export parse;

type version = {
    major: uint,
    minor: uint,
    patch: uint,
    tag: option<str>,
};

fn read_whitespace(rdr: io::reader, +ch: char) -> char {
    while ch == ' ' { ch = rdr.read_char(); }
    ch
}

fn parse_reader(rdr: io::reader) -> option<(version, char)> {
    fn read_digits(rdr: io::reader, -ch: char) -> option<(uint, char)> {
        let mut buf = "";

        while ch != -1 as char {
            alt ch {
              '0' to '9' { str::push_char(buf, ch); }
              _ { break; }
            }

            ch = rdr.read_char();
        }

        uint::from_str(buf).chain { |i| some((i, ch)) }
    }

    fn read_tag(rdr: io::reader) -> option<(str, char)> {
        let mut ch = rdr.read_char();
        let mut buf = "";

        while ch != -1 as char {
            alt ch {
              '0' to '9' | 'A' to 'Z' | 'a' to 'z' | '-' {
                str::push_char(buf, ch);
              }
              _ { break; }
            }
            ch = rdr.read_char();
        }

        if buf == "" { ret none; } else { some((buf, ch)) }
    }

    let ch = read_whitespace(rdr, rdr.read_char());

    let (major, ch) = alt read_digits(rdr, ch) {
      none { ret none; }
      some(item) { item }
    };

    if ch != '.' { ret none; }

    let (minor, ch) = alt read_digits(rdr, rdr.read_char()) {
      none { ret none; }
      some(item) { item }
    };

    if ch != '.' { ret none; }

    let (patch, ch) = alt read_digits(rdr, rdr.read_char()) {
      none { ret none; }
      some(item) { item }
    };

    let (tag, ch) = if ch == '-' {
        alt read_tag(rdr) {
          none { ret none; }
          some((tag, ch)) { (some(tag), ch) }
        }
    } else {
        (none, ch)
    };

    some(({ major: major, minor: minor, patch: patch, tag: tag }, ch))
}

fn parse(s: str) -> option<version> {
    io::with_str_reader(s) { |rdr|
        parse_reader(rdr).chain { |item|
            let (version, ch) = item;
            if read_whitespace(rdr, ch) != -1 as char {
                none
            } else {
                some(version)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        assert parse("") == none;
        assert parse("  ") == none;
        assert parse("1") == none;
        assert parse("1.2") == none;
        assert parse("1.2") == none;
        assert parse("1") == none;
        assert parse("1.2") == none;
        assert parse("1.2.3-") == none;
        assert parse("a.b.c") == none;
        assert parse("1.2.3 abc") == none;

        assert parse("1.2.3") == some({
            major: 1u,
            minor: 2u,
            patch: 3u,
            tag: none,
        });
        assert parse("  1.2.3  ") == some({
            major: 1u,
            minor: 2u,
            patch: 3u,
            tag: none,
        });
        assert parse("1.2.3-alpha1") == some({
            major: 1u,
            minor: 2u,
            patch: 3u,
            tag: some("alpha1")
        });
        assert parse("  1.2.3-alpha1  ") == some({
            major: 1u,
            minor: 2u,
            patch: 3u,
            tag: some("alpha1")
        });
    }

    #[test]
    fn test_eq() {
        assert parse("1.2.3")        == parse("1.2.3");
        assert parse("1.2.3-alpha1") == parse("1.2.3-alpha1");
    }

    #[test]
    fn test_ne() {
        assert parse("0.0.0")       != parse("0.0.1");
        assert parse("0.0.0")       != parse("0.1.0");
        assert parse("0.0.0")       != parse("1.0.0");
        assert parse("1.2.3-alpha") != parse("1.2.3-beta");
    }

    #[test]
    fn test_lt() {
        assert parse("0.0.0")        < parse("1.2.3-alpha2");
        assert parse("1.0.0")        < parse("1.2.3-alpha2");
        assert parse("1.2.0")        < parse("1.2.3-alpha2");
        assert parse("1.2.3")        < parse("1.2.3-alpha2");
        assert parse("1.2.3-alpha1") < parse("1.2.3-alpha2");

        assert !(parse("1.2.3-alpha2") < parse("1.2.3-alpha2"));
    }

    #[test]
    fn test_le() {
        assert parse("0.0.0")        <= parse("1.2.3-alpha2");
        assert parse("1.0.0")        <= parse("1.2.3-alpha2");
        assert parse("1.2.0")        <= parse("1.2.3-alpha2");
        assert parse("1.2.3")        <= parse("1.2.3-alpha2");
        assert parse("1.2.3-alpha1") <= parse("1.2.3-alpha2");
        assert parse("1.2.3-alpha2") <= parse("1.2.3-alpha2");
    }

    #[test]
    fn test_gt() {
        assert parse("1.2.3-alpha2") > parse("0.0.0");
        assert parse("1.2.3-alpha2") > parse("1.0.0");
        assert parse("1.2.3-alpha2") > parse("1.2.0");
        assert parse("1.2.3-alpha2") > parse("1.2.3");
        assert parse("1.2.3-alpha2") > parse("1.2.3-alpha1");

        assert !(parse("1.2.3-alpha2") > parse("1.2.3-alpha2"));
    }

    #[test]
    fn test_ge() {
        assert parse("1.2.3-alpha2") >= parse("0.0.0");
        assert parse("1.2.3-alpha2") >= parse("1.0.0");
        assert parse("1.2.3-alpha2") >= parse("1.2.0");
        assert parse("1.2.3-alpha2") >= parse("1.2.3");
        assert parse("1.2.3-alpha2") >= parse("1.2.3-alpha1");
        assert parse("1.2.3-alpha2") >= parse("1.2.3-alpha2");
    }
}
