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
    pub(crate) fn new(data: [u8; 1024]) -> Result<Self, &'static HsOpError> {
        // Ensure this is a valid packet by checking it starts with #hsop
        let mut header = [0; 6];
        header.copy_from_slice(&data[0..6]);
        let header = String::from_utf8(header.to_vec()).unwrap();
        if data.len() < 6 || header != "#hsop\0" {
            return Err(&HsOpError { details: "Invalid HandshakeOperation header" });
        }

        // Read the next byte to determine the operation
        let op = data[6];

        // Read the rest of the packet to get the payload
        let payload_data = &data[7..];

        Ok(HsOp {
            id: op,
            data: payload_data.to_vec(),
        })
    }

    pub(crate) fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend("#hsop\0".as_bytes());
        bytes.push(self.id);
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

