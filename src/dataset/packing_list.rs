use std::{
	fs::File,
	path::{Path, PathBuf},
};

use fnv::FnvHashMap;
use regex::Regex;

lazy_static::lazy_static! {
	pub(super) static ref TABLE_INFORMATION_RE: Regex =
		Regex::new(r"^(?P<table>[A-Za-z0-9]+)\|(?P<loc>[\d: ]+)\|$")
			.expect("regex parse failed");

	pub(super) static ref TABLE_INFORMATION_RE_ML: Regex =
		Regex::new(r"(?m)^(?P<table>[A-Za-z0-9]+)\|(?P<loc>[\d: ]+)\|$")
			.expect("regex parse failed");

	pub(super) static ref FILE_INFORMATION_RE: Regex =
		Regex::new(r"^(?P<filename>(?P<stusab>[a-z]{2})(?P<ident>\w+)(?P<year>\d{4})\.(?P<ds>.+))\|(?P<date>.+)\|(?P<size>\d+)\|(?P<lines>\d+)\|$")
			.expect("regex parse failed");

	pub(super) static ref FILE_INFORMATION_RE_ML: Regex =
		Regex::new(r"(?m)^(?P<filename>(?P<stusab>[a-z]{2})(?P<ident>\w+)(?P<year>\d{4})\.(?P<ds>.+))\|(?P<date>.+)\|(?P<size>\d+)\|(?P<lines>\d+)\|$")
			.expect("regex parse failed");

	pub(super) static ref STUSAB_RE: Regex =
		Regex::new(r"(?m)STUSAB: (?P<stusab>[A-Z]{2})$")
			.expect("regex parse failed");
}

use crate::{
	census2010, census2020, Result, Schema, Table, TableLocations, TableSegmentLocation,
	TableSegmentSpecifier,
};

pub struct PackingList {
	schema: Schema,
	directory: Option<PathBuf>,
	table_locations: FnvHashMap<Table, TableLocations>,
	tabular_files: FnvHashMap<u32, PathBuf>,
	geographical_header_file: PathBuf,
	rows: usize,
}

fn read_file_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
	use std::io::Read;

	let mut file = File::open(&path)?;
	let mut data = String::new();
	file.read_to_string(&mut data)?;

	Ok(data)
}

impl PackingList {
	pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self> {
		use core::str::FromStr;

		// It's generally quite a bit faster to just load the entire packing list
		// to a file and then do in-memory operations than deal with potential disk
		// buffering issues, so we first load to string.
		let data: String = read_file_to_string(&file_path)?;

		let mut parsed: Self = Self::from_str(&data)?;
		parsed.directory = file_path.as_ref().parent().map(ToOwned::to_owned);

		Ok(parsed)
	}

	pub fn new(
		schema: Schema,
		directory: Option<PathBuf>,
		table_locations: FnvHashMap<Table, TableLocations>,
		tabular_files: FnvHashMap<u32, PathBuf>,
		geographical_header_file: PathBuf,
		rows: usize,
	) -> Self {
		Self {
			schema,
			directory,
			table_locations,
			tabular_files,
			geographical_header_file,
			rows,
		}
	}

	pub fn schema(&self) -> Schema {
		self.schema
	}

	pub fn directory(&self) -> &Option<PathBuf> {
		&self.directory
	}

	pub fn table_locations(&self) -> &FnvHashMap<Table, TableLocations> {
		&self.table_locations
	}

	pub fn tabular_files(&self) -> &FnvHashMap<u32, PathBuf> {
		&self.tabular_files
	}

	pub fn geographical_header_file(&self) -> &PathBuf {
		&self.geographical_header_file
	}

	pub fn rows(&self) -> &usize {
		&self.rows
	}

	/// Find the file relative to the packing list's `directory` field.
	pub fn locate<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf> {
		self.directory.as_ref().map(|pb| pb.join(path))
	}
}

fn get_stusab(s: &str) -> String {
	let stusab_all_caps: &str = &STUSAB_RE.captures(s).expect("failed to find STUSAB")["stusab"];
	let stusab: String = stusab_all_caps.to_lowercase();
	stusab
}

fn extract_schemas(re: &Regex, s: &str) -> Vec<Schema> {
	re.captures_iter(s)
		.map(|captures| {
			log::trace!(
				"Processing filename regex match: {}",
				captures.get(0).unwrap().as_str()
			);

			let captures: (Option<&str>, Option<&str>, Option<&str>) = (
				captures.name("inner").as_ref().map(regex::Match::as_str),
				captures.name("year").as_ref().map(regex::Match::as_str),
				captures.name("ext").as_ref().map(regex::Match::as_str),
			);

			match captures {
				(Some(_), Some("2010"), Some("pl")) => Schema::Census2010(census2010::Schema::Pl94_171),
				_ => unimplemented!(),
			}
		})
		.collect()
}

#[derive(Debug, PartialEq)]
enum FileType {
	Tabular(u32),
	GeographicalHeader,
}

struct DataSegmentation {
	table_name: String,
	table_locations: Vec<TableLocations>,
}

#[derive(Debug)]
struct FileInformation {
	filename: PathBuf,
	date: String,
	file_size: usize,
	rows: usize,
	ty: FileType,
}

fn extract_file_information(s: &str) -> Vec<FileInformation> {
	FILE_INFORMATION_RE_ML
		.captures_iter(s)
		.map(|captures| {
			log::trace!(
				"Processing file information regex match: {}",
				captures.get(0).unwrap().as_str()
			);

			let (filename, date, size, rows): (&str, &str, &str, &str) = (
				captures
					.name("filename")
					.expect("missing capture group filename")
					.as_str(),
				captures
					.name("date")
					.expect("missing capture group date")
					.as_str(),
				captures
					.name("size")
					.expect("missing capture group size")
					.as_str(),
				captures
					.name("lines")
					.expect("missing capture group lines")
					.as_str(),
			);
			let filename = filename.to_string().into();
			let date = date.to_string();
			let file_size: usize = size.parse().expect("couldn't parse size as usize");
			let rows: usize = rows.parse().expect("couldn't parse rows as usize");
			let ty: FileType = match captures
				.name("ident")
				.expect("missing capture group ident")
				.as_str()
			{
				"geo" => FileType::GeographicalHeader,
				n => {
					if let Ok(idx) = n.parse::<u32>() {
						FileType::Tabular(idx)
					} else {
						unimplemented!()
					}
				}
			};

			log::trace!("Inferred filetype {:?} for {:?}", ty, filename);

			FileInformation {
				filename,
				date,
				file_size,
				rows,
				ty,
			}
		})
		.collect()
}

fn partition_file_information(
	file_informations: &[FileInformation],
) -> (FnvHashMap<u32, &FileInformation>, &FileInformation) {
	let header: &FileInformation = file_informations
		.iter()
		.find(|fi| fi.ty == FileType::GeographicalHeader)
		.expect("missing geographical header");
	let tabular_files: FnvHashMap<u32, &FileInformation> = file_informations
		.iter()
		.filter_map(|fi| match fi.ty {
			FileType::Tabular(idx) => Some((idx, fi)),
			_ => None,
		})
		.collect();

	(tabular_files, header)
}

fn convert_file_information(
	partition: &(FnvHashMap<u32, &FileInformation>, &FileInformation),
) -> (FnvHashMap<u32, PathBuf>, PathBuf, usize) {
	(
		partition
			.0
			.iter()
			.map(|(idx, fi)| (*idx, fi.filename.clone()))
			.collect(),
		partition.1.filename.clone(),
		{
			let mut vec: Vec<usize> = partition
				.0
				.iter()
				.map(|(_idx, fi)| fi.rows)
				.collect::<Vec<usize>>();
			vec.push(partition.1.rows);
			vec.dedup();
			debug_assert!(vec.len() == 1);
			vec[0]
		},
	)
}

fn extract_table_locations(schema: Schema, s: &str) -> FnvHashMap<Table, TableLocations> {
	let mut current_columns: FnvHashMap<u32, usize> = FnvHashMap::default();

	TABLE_INFORMATION_RE_ML
		.captures_iter(s)
		.map(|captures| -> (Table, TableLocations) {
			log::trace!(
				"Processing table segmentation regex match: {}",
				captures.get(0).unwrap().as_str()
			);

			let (name, specs): (&str, &str) = (
				captures
					.name("table")
					.expect("missing capture group table")
					.as_str(),
				captures
					.name("loc")
					.expect("missing capture group loc")
					.as_str(),
			);
			let specs: Vec<&str> = specs.split(' ').collect();
			let specs: Vec<TableSegmentSpecifier> = specs.iter().filter_map(|s| s.parse().ok()).collect();

			let table: Table = match (schema, name) {
				(Schema::Census2010(census2010::Schema::Pl94_171), "p1") => {
					Table::Census2010(census2010::Table::Pl94_171(census2010::pl94_171::P1))
				}
				(Schema::Census2010(census2010::Schema::Pl94_171), "p2") => {
					Table::Census2010(census2010::Table::Pl94_171(census2010::pl94_171::P2))
				}
				(Schema::Census2010(census2010::Schema::Pl94_171), "p3") => {
					Table::Census2010(census2010::Table::Pl94_171(census2010::pl94_171::P3))
				}
				(Schema::Census2010(census2010::Schema::Pl94_171), "p4") => {
					Table::Census2010(census2010::Table::Pl94_171(census2010::pl94_171::P4))
				}
				(Schema::Census2010(census2010::Schema::Pl94_171), "h1") => {
					Table::Census2010(census2010::Table::Pl94_171(census2010::pl94_171::H1))
				}
				(Schema::Census2010(census2010::Schema::Pl94_171), _) => unimplemented!(),

				(Schema::Census2020(census2020::Schema::Pl94_171), "p1") => {
					Table::Census2020(census2020::Table::Pl94_171(census2020::pl94_171::P1))
				}
				(Schema::Census2020(census2020::Schema::Pl94_171), "p2") => {
					Table::Census2020(census2020::Table::Pl94_171(census2020::pl94_171::P2))
				}
				(Schema::Census2020(census2020::Schema::Pl94_171), "p3") => {
					Table::Census2020(census2020::Table::Pl94_171(census2020::pl94_171::P3))
				}
				(Schema::Census2020(census2020::Schema::Pl94_171), "p4") => {
					Table::Census2020(census2020::Table::Pl94_171(census2020::pl94_171::P4))
				}
				(Schema::Census2020(census2020::Schema::Pl94_171), "h1") => {
					Table::Census2020(census2020::Table::Pl94_171(census2020::pl94_171::H1))
				}
				(Schema::Census2020(census2020::Schema::Pl94_171), "p5") => {
					Table::Census2020(census2020::Table::Pl94_171(census2020::pl94_171::P5))
				}
				(Schema::Census2020(census2020::Schema::Pl94_171), _) => unimplemented!(),
			};

			let locations: TableLocations = specs
				.iter()
				.map(|specifier| {
					if current_columns.get(&specifier.file).is_none() {
						current_columns.insert(specifier.file, 5_usize);
					}

					let start: usize = *current_columns.get(&specifier.file).unwrap();
					let end: usize = start + specifier.columns;

					current_columns.insert(specifier.file, end);

					TableSegmentLocation {
						file: specifier.file,
						range: start..end,
					}
				})
				.collect();

			log::trace!("Table {:?} is found at {:?}", table, locations);

			(table, locations)
		})
		.collect()
}

impl core::str::FromStr for PackingList {
	type Err = crate::error::Error;

	fn from_str(s: &str) -> Result<Self> {
		log::debug!("Parsing packing list (of {} bytes)", s.len());

		// PHASE 0: Build the regex for the stusab

		log::debug!("Parsing STUSAB field from packing list data");

		let stusab: String = get_stusab(s);

		log::debug!("Inferred STUSAB: {}", stusab);

		// PHASE 1: Collect the Schema

		log::trace!("Compiling filename regex");

		// TODO rather than building this regex at parse time, maybe store a lazy_static cache of these for each stusab somewhere?
		let filename_re: Regex = Regex::new(&format!(
			r"(?m){}(?P<inner>\w*)(?P<year>\d{{4}})\.(?P<ext>[a-z1-9\-]*)\b",
			stusab
		))
		.expect("failed to create generated regex");

		log::trace!("Finished compiling filename regex");

		log::debug!("Inferring schema");

		let mut schemas: Vec<Schema> = extract_schemas(&filename_re, s);

		log::trace!("Deduplicating {} schemas", schemas.len());

		schemas.dedup();

		log::trace!("Now have {} schema(s)", schemas.len());

		assert_eq!(schemas.len(), 1);

		let schema: Schema = schemas.remove(0);

		log::debug!("Inferred schema: {:?}", schema);

		// PHASE 2: Collect the file information

		log::debug!("Reading packing list content definitions");

		let (tabular_files, geographical_header_file, rows): (
			FnvHashMap<u32, PathBuf>,
			PathBuf,
			usize,
		) = {
			let file_informations: Vec<FileInformation> = extract_file_information(s);
			convert_file_information(&partition_file_information(&file_informations))
		};

		log::debug!(
			"Packing list: tabulars={:?}, header={:?}, rows={:?}",
			tabular_files,
			geographical_header_file,
			rows
		);

		// PHASE 3: Calculate the table locations

		log::debug!("Reading data segmentation specifiers");

		// TODO consider just hard-coding the table locations in our spec

		let table_locations: FnvHashMap<Table, TableLocations> = extract_table_locations(schema, s);

		Ok(Self {
			schema,
			directory: None,
			table_locations,
			tabular_files,
			geographical_header_file,
			rows,
		})
	}
}

#[cfg(test)]
mod tests {
	use crate::{Schema, Table};

	use super::PackingList;

	macro_rules! t_census2010_pl94_171 {
		($filename:literal, $stusab:ident) => {
			#[cfg(test)]
			mod $stusab {
				use super::{PackingList, Schema, Table};

				#[test]
				fn file_parses_and_is_as_expected() {
					let data = include_str!($filename);

					let packing_list: PackingList = data.parse().unwrap();
					assert_eq!(
						packing_list.schema,
						Schema::Census2010(crate::census2010::Schema::Pl94_171)
					);

					assert!(packing_list.tabular_files.len() == 2);

					assert!(packing_list.table_locations.len() == 5);
					assert!(
						packing_list
							.table_locations
							.get(&Table::Census2010(crate::census2010::Table::Pl94_171(
								crate::census2010::pl94_171::Table::P1
							)))
							.expect("missing mapping for c2010-P1")
							== &vec![crate::TableSegmentLocation {
								file: 1,
								range: 5..(5 + 71)
							}]
					);
					assert!(
						packing_list
							.table_locations
							.get(&Table::Census2010(crate::census2010::Table::Pl94_171(
								crate::census2010::pl94_171::Table::P2
							)))
							.expect("missing mapping for c2010-P2")
							== &vec![crate::TableSegmentLocation {
								file: 1,
								range: (5 + 71)..(5 + 71 + 73)
							}]
					);
					assert!(
						packing_list
							.table_locations
							.get(&Table::Census2010(crate::census2010::Table::Pl94_171(
								crate::census2010::pl94_171::Table::P3
							)))
							.expect("missing mapping for c2010-P3")
							== &vec![crate::TableSegmentLocation {
								file: 2,
								range: 5..(5 + 71)
							}]
					);
					assert!(
						packing_list
							.table_locations
							.get(&Table::Census2010(crate::census2010::Table::Pl94_171(
								crate::census2010::pl94_171::Table::P4
							)))
							.expect("missing mapping for c2010-P4")
							== &vec![crate::TableSegmentLocation {
								file: 2,
								range: (5 + 71)..(5 + 71 + 73)
							}]
					);
					assert!(
						packing_list
							.table_locations
							.get(&Table::Census2010(crate::census2010::Table::Pl94_171(
								crate::census2010::pl94_171::Table::H1
							)))
							.expect("missing mapping for c2010-H1")
							== &vec![crate::TableSegmentLocation {
								file: 2,
								range: (5 + 71 + 73)..(5 + 71 + 73 + 3)
							}]
					);
				}
			}
		};
	}

	t_census2010_pl94_171!("t/2010/ak2010.pl.prd.packinglist.txt", ak);
	t_census2010_pl94_171!("t/2010/al2010.pl.prd.packinglist.txt", al);
	t_census2010_pl94_171!("t/2010/ar2010.pl.prd.packinglist.txt", ar);
	t_census2010_pl94_171!("t/2010/az2010.pl.prd.packinglist.txt", az);
	t_census2010_pl94_171!("t/2010/ca2010.pl.prd.packinglist.txt", ca);
	t_census2010_pl94_171!("t/2010/co2010.pl.prd.packinglist.txt", co);
	t_census2010_pl94_171!("t/2010/ct2010.pl.prd.packinglist.txt", ct);
	t_census2010_pl94_171!("t/2010/dc2010.pl.prd.packinglist.txt", dc);
	t_census2010_pl94_171!("t/2010/de2010.pl.prd.packinglist.txt", de);
	t_census2010_pl94_171!("t/2010/fl2010.pl.prd.packinglist.txt", fl);
	t_census2010_pl94_171!("t/2010/ga2010.pl.prd.packinglist.txt", ga);
	t_census2010_pl94_171!("t/2010/hi2010.pl.prd.packinglist.txt", hi);
	t_census2010_pl94_171!("t/2010/ia2010.pl.prd.packinglist.txt", ia);
	t_census2010_pl94_171!("t/2010/id2010.pl.prd.packinglist.txt", id);
	t_census2010_pl94_171!("t/2010/il2010.pl.prd.packinglist.txt", il);
	t_census2010_pl94_171!("t/2010/in2010.pl.prd.packinglist.txt", r#in);
	t_census2010_pl94_171!("t/2010/ks2010.pl.prd.packinglist.txt", ks);
	t_census2010_pl94_171!("t/2010/ky2010.pl.prd.packinglist.txt", ky);
	t_census2010_pl94_171!("t/2010/la2010.pl.prd.packinglist.txt", la);
	t_census2010_pl94_171!("t/2010/ma2010.pl.prd.packinglist.txt", ma);
	t_census2010_pl94_171!("t/2010/md2010.pl.prd.packinglist.txt", md);
	t_census2010_pl94_171!("t/2010/me2010.pl.prd.packinglist.txt", me);
	t_census2010_pl94_171!("t/2010/mi2010.pl.prd.packinglist.txt", mi);
	t_census2010_pl94_171!("t/2010/mn2010.pl.prd.packinglist.txt", mn);
	t_census2010_pl94_171!("t/2010/mo2010.pl.prd.packinglist.txt", mo);
	t_census2010_pl94_171!("t/2010/ms2010.pl.prd.packinglist.txt", ms);
	t_census2010_pl94_171!("t/2010/mt2010.pl.prd.packinglist.txt", mt);
	t_census2010_pl94_171!("t/2010/nc2010.pl.prd.packinglist.txt", nc);
	t_census2010_pl94_171!("t/2010/nd2010.pl.prd.packinglist.txt", nd);
	t_census2010_pl94_171!("t/2010/ne2010.pl.prd.packinglist.txt", ne);
	t_census2010_pl94_171!("t/2010/nh2010.pl.prd.packinglist.txt", nh);
	t_census2010_pl94_171!("t/2010/nj2010.pl.prd.packinglist.txt", nj);
	t_census2010_pl94_171!("t/2010/nm2010.pl.prd.packinglist.txt", nm);
	t_census2010_pl94_171!("t/2010/nv2010.pl.prd.packinglist.txt", nv);
	t_census2010_pl94_171!("t/2010/ny2010.pl.prd.packinglist.txt", ny);
	t_census2010_pl94_171!("t/2010/oh2010.pl.prd.packinglist.txt", oh);
	t_census2010_pl94_171!("t/2010/ok2010.pl.prd.packinglist.txt", ok);
	t_census2010_pl94_171!("t/2010/or2010.pl.prd.packinglist.txt", or);
	t_census2010_pl94_171!("t/2010/pa2010.pl.prd.packinglist.txt", pa);
	t_census2010_pl94_171!("t/2010/pr2010.pl.prd.packinglist.txt", pr);
	t_census2010_pl94_171!("t/2010/ri2010.pl.prd.packinglist.txt", ri);
	t_census2010_pl94_171!("t/2010/sc2010.pl.prd.packinglist.txt", sc);
	t_census2010_pl94_171!("t/2010/sd2010.pl.prd.packinglist.txt", sd);
	t_census2010_pl94_171!("t/2010/tn2010.pl.prd.packinglist.txt", tn);
	t_census2010_pl94_171!("t/2010/tx2010.pl.prd.packinglist.txt", tx);
	t_census2010_pl94_171!("t/2010/ut2010.pl.prd.packinglist.txt", ut);
	t_census2010_pl94_171!("t/2010/va2010.pl.prd.packinglist.txt", va);
	t_census2010_pl94_171!("t/2010/vt2010.pl.prd.packinglist.txt", vt);
	t_census2010_pl94_171!("t/2010/wa2010.pl.prd.packinglist.txt", wa);
	t_census2010_pl94_171!("t/2010/wi2010.pl.prd.packinglist.txt", wi);
	t_census2010_pl94_171!("t/2010/wv2010.pl.prd.packinglist.txt", wv);
	t_census2010_pl94_171!("t/2010/wy2010.pl.prd.packinglist.txt", wy);
}
