use std::fmt;

pub(crate) struct OscMessage {
    pub(crate) address: String,
    pub(crate) type_tag: char,
    pub(crate) data: Vec<u8>,
}

#[derive(Debug)]
pub(crate) struct OscError {
    pub(crate) details: &'static str
}

impl fmt::Display for OscError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing OSC message: {}", self.details)
    }
}

fn quantize(value: &mut usize, quant: usize) {
    if *value % quant != 0 {
        *value += quant - (*value % quant);
    }
}

impl OscMessage {
    pub(crate) fn new(data: &Vec<u8>) -> Result<Self, &'static OscError> {
        // Read until the first null byte to get the address and ensure it starts with /
        let mut address_bytes = [0; 1024];
        let mut i = 0;
        while data[i] != 0 {
            address_bytes[i] = data[i];
            i += 1;
        }
        i += 1; // Account for null term

        let address = match String::from_utf8(address_bytes.to_vec()) {
            Ok(s) => {
                if !s.starts_with("/") {
                    return Err(&OscError { details: "Invalid OSC address" });
                }
                s
            },
            Err(_) => {
                return Err(&OscError { details: "OSC address must be valid UTF-8" });
            }
        };

        // Align the index to the next multiple of 4 if it isn't already
        quantize(&mut i, 4);

        // Read the next byte and ensure it's a comma, then read the next and store it as a type tag
        if data[i] != b',' {
            return Err(&OscError { details: "Invalid OSC type tag" });
        }
        i += 1;

        // Read the type tag
        let type_tag = data[i] as char;
        i += 3;

        // Include the rest of the data
        let data = data[i..].to_vec();

        Ok(OscMessage {
            address,
            type_tag,
            data,
        })
    }

    pub(crate) fn serialize(&self) -> Vec<u8> {
        unimplemented!()
    }
}