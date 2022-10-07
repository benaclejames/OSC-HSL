use std::fmt;

#[derive(Debug)]
pub(crate) struct HsOpError {
    pub(crate) details: &'static str
}

impl fmt::Display for HsOpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing OSC message: {}", self.details)
    }
}

pub(crate) trait HsSerializable {
    fn serialize(&self) -> Vec<u8>;
    fn new(data: Vec<u8>) -> Result<Self, HsOpError> where Self: Sized;
}

pub(crate) struct AppInfo<'a> {
    pub(crate) id: &'a str,
    pub(crate) friendly_name: &'a str,
    pub(crate) version: &'a str,
}

pub(crate) struct HsOp {
    pub(crate) id: u8,
    pub(crate) data: Vec<u8>,
}

impl HsOp {
    const HEADER: &'static str = "#hsop\0\0\0"; // Kinda ugly, but all elements need to be quantized to 4 bytes

    pub(crate) fn new(data: &Vec<u8>) -> Result<Self, &'static HsOpError> {
        // Ensure this is a valid packet by checking it starts with #hsop
        let mut header = [0; 8];
        header.copy_from_slice(&data[0..8]);
        let header = String::from_utf8(header.to_vec()).unwrap();
        if header != HsOp::HEADER {
            return Err(&HsOpError { details: "Invalid HandshakeOperation header" });
        }

        // Read the next byte to ensure it's a comma, following our header format
        if data[8] != b',' {
            return Err(&HsOpError { details: "Invalid HandshakeOperation type tag" });
        }

        // Read the next byte to get the ID
        let op = data[9];

        // Read the rest of the packet to get the payload
        let payload_data = &data[10..];

        Ok(HsOp {
            id: op,
            data: payload_data.to_vec(),
        })
    }

    pub(crate) fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(HsOp::HEADER.as_bytes());
        bytes.push(b',');
        bytes.push(self.id);
        bytes.extend([0, 0]);
        bytes.extend(&self.data);
        bytes
    }
}

pub(crate) struct HsStatus<'a> {
    pub(crate) apps: Vec<AppInfo<'a>>,
    pub(crate) additional_data: Vec<u8>,
}

impl HsSerializable for HsStatus<'_> {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(0x00);
        data.extend(self.apps.iter().map(|app| {
            let mut app_data = Vec::new();
            app_data.extend(app.id.as_bytes());
            app_data.push(0x00);
            app_data.extend(app.friendly_name.as_bytes());
            app_data.push(0x00);
            app_data.extend(app.version.as_bytes());
            app_data.push(0x00);
            app_data
        }).flatten());
        data.append(&mut self.additional_data.clone());
        data
    }

    fn new(data: Vec<u8>) -> Result<Self, HsOpError> where Self: Sized {
        unimplemented!()
    }
}

