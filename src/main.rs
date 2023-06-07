// This code is going to be rough. Just trying to parse this using the technique
// I know: just play with the raw data
fn main() {
	let fname = std::env::args().nth(1).unwrap();
	let data = std::fs::read(fname).unwrap();

	println!("Read {:.2}MB", data.len() as f32 / (1024.0 * 1024.0));

	let header = LightHeader::new(&data[0..32]);
	let data = &data[32..];

	let LightHeader {
		magic_number,
		combined_length,
		header_length,
		message_length,
		kind,
		reserved,
	} = header;

	println!("\nMagic: {magic_number}\nCombined Length: {combined_length}\nHeader Length: {header_length}\nMessage Length: {message_length}\nKind: {kind}\nReserved: {reserved:?}\nNext 8 Bytes: {:?}", &data[0..8]);

	let message = &data[..message_length as usize];
	let data = &data[message_length as usize..];

	println!("\nTook Message of {message_length} bytes");
	println!("{} bytes left", data.len());

	println!("{:?}", &data[0..32]);
}

struct LightHeader {
	magic_number: String,
	combined_length: u64,
	header_length: u64,
	message_length: u32,
	// type
	kind: u8,
	reserved: [u8; 7],
}

impl LightHeader {
	pub fn new(data: &[u8]) -> Self {
		let magic_number = String::from_utf8(data[0..4].to_vec()).unwrap();
		let combined_length = u64::from_le_bytes(data[4..12].try_into().unwrap());
		println!("Combined Length: {:?}", &data[4..12]);

		let header_length = u64::from_le_bytes(data[12..20].try_into().unwrap());
		println!("Header Length: {:?}", &data[12..20]);

		let message_length = u32::from_le_bytes(data[20..24].try_into().unwrap());
		println!("Message Length: {:?}", &data[20..24]);

		let kind = data[24];
		let reserved = data[25..32].try_into().unwrap();

		LightHeader {
			magic_number,
			combined_length,
			header_length,
			message_length,
			kind,
			reserved,
		}
	}
}
