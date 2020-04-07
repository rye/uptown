use distringo::Dataset;

/// Simple loading example
///
/// Reads a packing list from
fn main() -> distringo::error::Result<()> {
	simple_logger::init_with_level(log::Level::Trace).unwrap();

	let ds = distringo::IndexedDataset::new("in2010-pl94_171")
		.unpack("data/in2010.pl.prd.packinglist.txt")?
		.index()?;

	let start = std::time::Instant::now();
	let string_record = ds.get_logical_record(
		0335180,
		vec![
			distringo::Schema::Census2010Pl94_171(Some(distringo::census2010::pl94_171::P1)),
			distringo::Schema::Census2010Pl94_171(Some(distringo::census2010::pl94_171::P2)),
			distringo::Schema::Census2010Pl94_171(Some(distringo::census2010::pl94_171::P3)),
			distringo::Schema::Census2010Pl94_171(Some(distringo::census2010::pl94_171::P4)),
			distringo::Schema::Census2010Pl94_171(Some(distringo::census2010::pl94_171::H1)),
		],
	)?;

	println!(
		"Retrieved record {:?} in {}ns",
		string_record,
		std::time::Instant::now().duration_since(start).as_nanos()
	);

	assert_eq!(
		string_record,
		csv::StringRecord::from(vec![
			"53", "52", "50", "0", "0", "2", "0", "0", "1", "1", "0", "0", "1", "0", "0", "0", "0", "0",
			"0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0",
			"0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0",
			"0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "53",
			"2", "51", "50", "48", "0", "0", "2", "0", "0", "1", "1", "0", "0", "1", "0", "0", "0", "0",
			"0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0",
			"0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0",
			"0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0",
			"45", "45", "43", "0", "0", "2", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0",
			"0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0",
			"0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0",
			"0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "45",
			"1", "44", "44", "42", "0", "0", "2", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0",
			"0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0",
			"0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0",
			"0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0",
			"24", "24", "0"
		])
	);

	let logrecno = ds.get_logical_record_number_for_geoid("181570052001013")?;

	assert_eq!(logrecno, 0335180);

	let header = ds.get_header_for_geoid("181570052001013")?;
	println!("{}", header.name());

	assert_eq!(header.logrecno(), 0335180);

	Ok(())
}
