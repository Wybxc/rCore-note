use crate::print;

const FD_STDOUT: usize = 1;

fn from_utf8_lossy<F>(mut input: &[u8], mut push: F)
where
    F: FnMut(&str),
{
    loop {
        match core::str::from_utf8(input) {
            Ok(valid) => {
                push(valid);
                break;
            }
            Err(error) => {
                let (valid, after_valid) = input.split_at(error.valid_up_to());
                unsafe { push(core::str::from_utf8_unchecked(valid)) }
                // push("\u{FFFD}");
                push("⍰");

                if let Some(invalid_sequence_length) = error.error_len() {
                    input = &after_valid[invalid_sequence_length..]
                } else {
                    break;
                }
            }
        }
    }
}

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            from_utf8_lossy(slice, |s| {
                print!("{}", s);
            });
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}
