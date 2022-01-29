use core::iter::Iterator;

pub struct SundayMatcher<'a> {
    haytax: &'a str,
    needle: &'a str,
    map: [Option<usize>; 255],
    position: usize,
}

impl<'a> SundayMatcher<'a> {
    pub fn new(haytax: &'a str, needle: &'a str) -> Self {
        let mut matcher = Self {
            haytax,
            needle,
            map: [None; 255],
            position: 0,
        };

        matcher.init();

        matcher
    }

    fn init(&mut self) {
        let needle = self.needle.as_bytes();
        needle.iter().enumerate().for_each(|(index, item)| {
            let item_index = *item as usize;
            self.map[item_index] = Some(index);
        });
    }
}

impl<'a> Iterator for SundayMatcher<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let haytax = {
            let haytax = self.haytax.as_bytes();
            &haytax[self.position..]
        };
        let needle = self.needle.as_bytes();
        let haytax_len = haytax.len();
        let needle_len = needle.len();

        if haytax.is_empty() && needle.is_empty() {
            return Some((0, self.needle));
        }

        if haytax_len < needle_len {
            return None;
        }

        let mut position = 0;

        while position + needle_len <= haytax_len {
            let jump = position + needle_len;
            if &haytax[position..jump] == needle {
                let haytax_position = self.position + position;
                self.position += jump;
                return Some((haytax_position, self.needle));
            } else if jump == haytax_len {
                self.position += jump;
                return None;
            } else {
                let index = haytax[jump] as usize;
                match self.map[index] {
                    Some(index) => position += needle_len - index,
                    None => position += needle_len + 1,
                }
            }
        }

        self.position += position + needle_len;
        None
    }
}

#[test]
fn test1() {
    let matcher = SundayMatcher::new("mississippi", "issi");
    let std_matcher = "mississippi".match_indices("issi");
    matcher.into_iter()
        .zip(std_matcher)
        .for_each(|(item1, item2)| {
            assert_eq!(item1, item2);
        })
}
