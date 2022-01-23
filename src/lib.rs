pub fn sunday_match(source: &str, target: &str) -> Option<usize> {
    let source = source.as_bytes();
    let target = target.as_bytes();

    let source_len = source.len();
    let target_len = target.len();
    let mut source_position = 0;

    let mut map = [None; u8::MAX as usize];

    for (index, item) in target.iter().enumerate() {
        let item_index = *item as usize;
        map[item_index] = Some(index);
    }

    while source_position < source_len {
        if source_len - source_position < target_len {
            return None;
        }

        let tmp = unsafe { source.get_unchecked(source_position..source_position + target_len) }
            == target;

        if tmp {
            return Some(source_position);
        }

        let jump = source_position + target_len;
        if jump < source_len {
            match map[*unsafe { source.get_unchecked(jump) } as usize] {
                Some(position) => source_position += target_len - position,
                None => source_position += target_len,
            }
        } else {
            break;
        }
    }
    None
}