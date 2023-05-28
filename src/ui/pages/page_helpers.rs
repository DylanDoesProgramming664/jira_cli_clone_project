use std::cmp::{max, min};

use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {
    let len = text.len();

    return match len.cmp(&width) {
        std::cmp::Ordering::Equal => text.to_owned(),
        std::cmp::Ordering::Less => {
            let diff = width - len;
            let mut new_text = text.to_owned();
            for _ in 0..diff {
                new_text.push(' ');
            }
            new_text
        }
        std::cmp::Ordering::Greater => match width {
            0 => "".to_owned(),
            1 => ".".to_owned(),
            2 => "..".to_owned(),
            3 => "...".to_owned(),
            _ => text.truncate_ellipse(width - 3).to_string(),
        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    }
}
