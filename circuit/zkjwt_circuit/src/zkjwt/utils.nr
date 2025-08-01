use crate::{MAX_EMAIL_LENGTH};

fn decimal_string_to_field<let LEN: u32>(decimal_bytes: [u8; LEN]) -> Field {
    assert(LEN <= 77);

    let mut field: Field = 0;
    let mut multiplier: Field = 1;

    for i in 0..LEN {
        let ascii_char = decimal_bytes[LEN - i - 1];
        if ascii_char >= 48 & ascii_char <= 57 {
            let digit = ascii_char as Field - 48;
            field += digit * multiplier;
            multiplier *= 10;
        }
    }

    field
}

unconstrained fn get_domain_start_index_in_email(email: BoundedVec<u8, MAX_EMAIL_LENGTH>) -> u32 {
    let mut domain_start_index = 0;
    for i in 0..MAX_EMAIL_LENGTH {
        if email.storage()[i] == ("@".as_bytes())[0] {
            domain_start_index = i + 1;
            break;
        }
    }

    domain_start_index
}