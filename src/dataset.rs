//! Dataset structures and year/dataset-specific implementations thereof
//!
//! This module contains the core generics and expectations, and then some of the implementations
//! of those generics.

/// A type that can locate, _on a given line_, where a field lives.
trait DelimiterlessLayout<F: Field> {
	fn get_span(field: &F) -> core::ops::Range<usize>;
}

/// A type that can locate,
trait ColumnLayout<F: Field> {
	fn locate_field(&self, field: &F) -> usize;
}

trait Layout<'input> {
	type Field;

	fn all_fields(data: &'input str) -> Vec<(&Self::Field, &'input str)>;
	fn all_fields_trimmed(data: &'input str) -> Vec<(&Self::Field, &'input str)>;
	fn fields(data: &'input str) -> Vec<(&Self::Field, &'input str)>;
}

// struct DelimiterlessLayout {
// }

// enum FileType {
// 	FixedColumnSize,
// 	// TODO Once #[feature(pattern)] is stabilized, use that instead?
// 	Delimited(char),
// }

// struct DatasetHeaderFile {
// 	ty: FileType,
// }

// struct DatasetDataFile {
// 	ty: FileType,
// }

// struct Dataset {
// 	header: DatasetHeaderFile,
// 	files: Vec<DatasetDataFile>
// }

trait Field {}

mod census2010 {
	mod pl94_171 {
		mod header {
			#[derive(Debug, PartialEq)]
			enum Field {
				FILEID,
				STUSAB,
				SUMLEV,
				GEOCOMP,
				CHARITER,
				CIFSN,
				LOGRECNO,
				REGION,
				DIVISION,
				STATE,
				COUNTY,
				COUNTYCC,
				COUNTYSC,
				COUSUB,
				COUSUBCC,
				COUSUBSC,
				PLACE,
				PLACECC,
				PLACESC,
				TRACT,
				BLKGRP,
				BLOCK,
				IUC,
				CONCIT,
				CONCITCC,
				CONCITSC,
				AIANHH,
				AIANHHFP,
				AIANHHCC,
				AIHHTLI,
				AITSCE,
				AITS,
				AITSCC,
				TTRACT,
				TBLKGRP,
				ANRC,
				ANRCCC,
				CBSA,
				CBASC,
				METDIV,
				CSA,
				NECTA,
				NECTASC,
				NECTADIV,
				CNECTA,
				CBSAPCI,
				NECTAPCI,
				UA,
				UASC,
				UATYPE,
				UR,
				CD,
				SLDU,
				SLDL,
				VTD,
				VTDI,
				RESERVE2,
				ZCTA5,
				SUBMCD,
				SUBMCDCC,
				SDELM,
				SDSEC,
				SDUNI,
				AREALAND,
				AREAWATR,
				NAME,
				FUNCSTAT,
				GCUNI,
				POP100,
				HU100,
				INTPTLAT,
				INTPTLON,
				LSADC,
				PARTFLAG,
				RESERVE3,
				UGA,
				STATENS,
				COUNTYNS,
				COUSUBNS,
				PLACENS,
				CONCITNS,
				AIANHHNS,
				AITSNS,
				ANRCNS,
				SUBMCDNS,
				CD113,
				CD114,
				CD115,
				SLDU2,
				SLDU3,
				SLDU4,
				SLDL2,
				SLDL3,
				SLDL4,
				AIANHHSC,
				CSASC,
				CNECTASC,
				MEMI,
				NMEMI,
				PUMA,
				RESERVED,
			}

			use Field::*;

			const FIELDS: &'static [Field] = &[
				FILEID, STUSAB, SUMLEV, GEOCOMP, CHARITER, CIFSN, LOGRECNO, REGION, DIVISION, STATE,
				COUNTY, COUNTYCC, COUNTYSC, COUSUB, COUSUBCC, COUSUBSC, PLACE, PLACECC, PLACESC, TRACT,
				BLKGRP, BLOCK, IUC, CONCIT, CONCITCC, CONCITSC, AIANHH, AIANHHFP, AIANHHCC, AIHHTLI,
				AITSCE, AITS, AITSCC, TTRACT, TBLKGRP, ANRC, ANRCCC, CBSA, CBASC, METDIV, CSA, NECTA,
				NECTASC, NECTADIV, CNECTA, CBSAPCI, NECTAPCI, UA, UASC, UATYPE, UR, CD, SLDU, SLDL, VTD,
				VTDI, RESERVE2, ZCTA5, SUBMCD, SUBMCDCC, SDELM, SDSEC, SDUNI, AREALAND, AREAWATR, NAME,
				FUNCSTAT, GCUNI, POP100, HU100, INTPTLAT, INTPTLON, LSADC, PARTFLAG, RESERVE3, UGA,
				STATENS, COUNTYNS, COUSUBNS, PLACENS, CONCITNS, AIANHHNS, AITSNS, ANRCNS, SUBMCDNS, CD113,
				CD114, CD115, SLDU2, SLDU3, SLDU4, SLDL2, SLDL3, SLDL4, AIANHHSC, CSASC, CNECTASC, MEMI,
				NMEMI, PUMA, RESERVED,
			];

			impl crate::dataset::Field for Field {}

			struct Pl94_171Layout {}

			impl crate::dataset::Layout<'_> for Pl94_171Layout {
				type Field = Field;

				fn all_fields<'input>(line: &'input str) -> Vec<(&Self::Field, &'input str)> {
					use crate::dataset::DelimiterlessLayout;
					FIELDS
						.iter()
						.map(|field| (field, &line[Self::get_span(field)]))
						.collect()
				}

				fn all_fields_trimmed<'input>(line: &'input str) -> Vec<(&Self::Field, &'input str)> {
					use crate::dataset::DelimiterlessLayout;
					FIELDS
						.iter()
						.map(|field| (field, line[Self::get_span(field)].trim()))
						.collect()
				}

				fn fields<'input>(s: &'input str) -> Vec<(&Self::Field, &'input str)> {
					use crate::dataset::DelimiterlessLayout;
					FIELDS
						.iter()
						.filter_map(|field| {
							let span = s[Self::get_span(field)].trim();

							if span.len() > 0 {
								Some((field, span))
							} else {
								None
							}
						})
						.collect()
				}
			}

			impl crate::dataset::DelimiterlessLayout<Field> for Pl94_171Layout {
				fn get_span(field: &Field) -> core::ops::Range<usize> {
					use Field::*;
					match field {
						FILEID => 0..6,
						STUSAB => 6..8,
						SUMLEV => 8..11,
						GEOCOMP => 11..13,
						CHARITER => 13..16,
						CIFSN => 16..18,
						LOGRECNO => 18..25,
						REGION => 25..26,
						DIVISION => 26..27,
						STATE => 27..29,
						COUNTY => 29..32,
						COUNTYCC => 32..34,
						COUNTYSC => 34..36,
						COUSUB => 36..41,
						COUSUBCC => 41..43,
						COUSUBSC => 43..45,
						PLACE => 45..50,
						PLACECC => 50..52,
						PLACESC => 52..54,
						TRACT => 54..60,
						BLKGRP => 60..61,
						BLOCK => 61..65,
						IUC => 65..67,
						CONCIT => 67..72,
						CONCITCC => 72..74,
						CONCITSC => 74..76,
						AIANHH => 76..80,
						AIANHHFP => 80..85,
						AIANHHCC => 85..87,
						AIHHTLI => 87..88,
						AITSCE => 88..91,
						AITS => 91..96,
						AITSCC => 96..98,
						TTRACT => 98..104,
						TBLKGRP => 104..105,
						ANRC => 105..110,
						ANRCCC => 110..112,
						CBSA => 112..117,
						CBASC => 117..119,
						METDIV => 119..124,
						CSA => 124..127,
						NECTA => 127..132,
						NECTASC => 132..134,
						NECTADIV => 134..139,
						CNECTA => 139..142,
						CBSAPCI => 142..143,
						NECTAPCI => 143..144,
						UA => 144..149,
						UASC => 149..151,
						UATYPE => 151..152,
						UR => 152..153,
						CD => 153..155,
						SLDU => 155..158,
						SLDL => 158..161,
						VTD => 161..167,
						VTDI => 167..168,
						RESERVE2 => 168..171,
						ZCTA5 => 171..176,
						SUBMCD => 176..181,
						SUBMCDCC => 181..183,
						SDELM => 183..188,
						SDSEC => 188..193,
						SDUNI => 193..198,
						AREALAND => 198..212,
						AREAWATR => 212..226,
						NAME => 226..316,
						FUNCSTAT => 316..317,
						GCUNI => 317..318,
						POP100 => 318..327,
						HU100 => 327..336,
						INTPTLAT => 336..347,
						INTPTLON => 347..359,
						LSADC => 359..361,
						PARTFLAG => 361..362,
						RESERVE3 => 362..368,
						UGA => 368..373,
						STATENS => 373..381,
						COUNTYNS => 381..389,
						COUSUBNS => 389..397,
						PLACENS => 397..405,
						CONCITNS => 405..413,
						AIANHHNS => 413..421,
						AITSNS => 421..429,
						ANRCNS => 429..437,
						SUBMCDNS => 437..445,
						CD113 => 445..447,
						CD114 => 447..449,
						CD115 => 449..451,
						SLDU2 => 451..454,
						SLDU3 => 454..457,
						SLDU4 => 457..460,
						SLDL2 => 460..463,
						SLDL3 => 463..466,
						SLDL4 => 466..469,
						AIANHHSC => 469..471,
						CSASC => 471..473,
						CNECTASC => 473..475,
						MEMI => 475..476,
						NMEMI => 476..477,
						PUMA => 477..482,
						RESERVED => 482..500,
					}
				}
			}

			#[cfg(test)]
			mod tests {
				use crate::dataset::Layout;

				use super::Field::*;

				const EXAMPLE_LINE: &str = "PLST  IN04000000  00000012318                                                                                                                                                                            92789193658    1537004191Indiana                                                                                   A!  6483802  2795541+39.9030256-086.283950300            00448508                                                                                                                       ";

				#[test]
				fn example_line_fields_trimmed() {
					let fields = super::Pl94_171Layout::all_fields_trimmed(EXAMPLE_LINE);

					assert_eq!(
						vec![
							(&FILEID, "PLST"),
							(&STUSAB, "IN"),
							(&SUMLEV, "040"),
							(&GEOCOMP, "00"),
							(&CHARITER, "000"),
							(&CIFSN, ""),
							(&LOGRECNO, "0000001"),
							(&REGION, "2"),
							(&DIVISION, "3"),
							(&STATE, "18"),
							(&COUNTY, ""),
							(&COUNTYCC, ""),
							(&COUNTYSC, ""),
							(&COUSUB, ""),
							(&COUSUBCC, ""),
							(&COUSUBSC, ""),
							(&PLACE, ""),
							(&PLACECC, ""),
							(&PLACESC, ""),
							(&TRACT, ""),
							(&BLKGRP, ""),
							(&BLOCK, ""),
							(&IUC, ""),
							(&CONCIT, ""),
							(&CONCITCC, ""),
							(&CONCITSC, ""),
							(&AIANHH, ""),
							(&AIANHHFP, ""),
							(&AIANHHCC, ""),
							(&AIHHTLI, ""),
							(&AITSCE, ""),
							(&AITS, ""),
							(&AITSCC, ""),
							(&TTRACT, ""),
							(&TBLKGRP, ""),
							(&ANRC, ""),
							(&ANRCCC, ""),
							(&CBSA, ""),
							(&CBASC, ""),
							(&METDIV, ""),
							(&CSA, ""),
							(&NECTA, ""),
							(&NECTASC, ""),
							(&NECTADIV, ""),
							(&CNECTA, ""),
							(&CBSAPCI, ""),
							(&NECTAPCI, ""),
							(&UA, ""),
							(&UASC, ""),
							(&UATYPE, ""),
							(&UR, ""),
							(&CD, ""),
							(&SLDU, ""),
							(&SLDL, ""),
							(&VTD, ""),
							(&VTDI, ""),
							(&RESERVE2, ""),
							(&ZCTA5, ""),
							(&SUBMCD, ""),
							(&SUBMCDCC, ""),
							(&SDELM, ""),
							(&SDSEC, ""),
							(&SDUNI, ""),
							(&AREALAND, "92789193658"),
							(&AREAWATR, "1537004191"),
							(&NAME, "Indiana"),
							(&FUNCSTAT, "A"),
							(&GCUNI, "!"),
							(&POP100, "6483802"),
							(&HU100, "2795541"),
							(&INTPTLAT, "+39.9030256"),
							(&INTPTLON, "-086.2839503"),
							(&LSADC, "00"),
							(&PARTFLAG, ""),
							(&RESERVE3, ""),
							(&UGA, ""),
							(&STATENS, "00448508"),
							(&COUNTYNS, ""),
							(&COUSUBNS, ""),
							(&PLACENS, ""),
							(&CONCITNS, ""),
							(&AIANHHNS, ""),
							(&AITSNS, ""),
							(&ANRCNS, ""),
							(&SUBMCDNS, ""),
							(&CD113, ""),
							(&CD114, ""),
							(&CD115, ""),
							(&SLDU2, ""),
							(&SLDU3, ""),
							(&SLDU4, ""),
							(&SLDL2, ""),
							(&SLDL3, ""),
							(&SLDL4, ""),
							(&AIANHHSC, ""),
							(&CSASC, ""),
							(&CNECTASC, ""),
							(&MEMI, ""),
							(&NMEMI, ""),
							(&PUMA, ""),
							(&RESERVED, "")
						],
						fields
					);
				}

				#[test]
				fn example_line_fields() {
					let fields = super::Pl94_171Layout::all_fields(EXAMPLE_LINE);

					assert_eq!(vec![
						(&FILEID, "PLST  "),
						(&STUSAB, "IN"),
						(&SUMLEV, "040"),
						(&GEOCOMP, "00"),
						(&CHARITER, "000"),
						(&CIFSN, "  "),
						(&LOGRECNO, "0000001"),
						(&REGION, "2"),
						(&DIVISION, "3"),
						(&STATE, "18"),
						(&COUNTY, "   "),
						(&COUNTYCC, "  "),
						(&COUNTYSC, "  "),
						(&COUSUB, "     "),
						(&COUSUBCC, "  "),
						(&COUSUBSC, "  "),
						(&PLACE, "     "),
						(&PLACECC, "  "),
						(&PLACESC, "  "),
						(&TRACT, "      "),
						(&BLKGRP, " "),
						(&BLOCK, "    "),
						(&IUC, "  "),
						(&CONCIT, "     "),
						(&CONCITCC, "  "),
						(&CONCITSC, "  "),
						(&AIANHH, "    "),
						(&AIANHHFP, "     "),
						(&AIANHHCC, "  "),
						(&AIHHTLI, " "),
						(&AITSCE, "   "),
						(&AITS, "     "),
						(&AITSCC, "  "),
						(&TTRACT, "      "),
						(&TBLKGRP, " "),
						(&ANRC, "     "),
						(&ANRCCC, "  "),
						(&CBSA, "     "),
						(&CBASC, "  "),
						(&METDIV, "     "),
						(&CSA, "   "),
						(&NECTA, "     "),
						(&NECTASC, "  "),
						(&NECTADIV, "     "),
						(&CNECTA, "   "),
						(&CBSAPCI, " "),
						(&NECTAPCI, " "),
						(&UA, "     "),
						(&UASC, "  "),
						(&UATYPE, " "),
						(&UR, " "),
						(&CD, "  "),
						(&SLDU, "   "),
						(&SLDL, "   "),
						(&VTD, "      "),
						(&VTDI, " "),
						(&RESERVE2, "   "),
						(&ZCTA5, "     "),
						(&SUBMCD, "     "),
						(&SUBMCDCC, "  "),
						(&SDELM, "     "),
						(&SDSEC, "     "),
						(&SDUNI, "     "),
						(&AREALAND, "   92789193658"),
						(&AREAWATR, "    1537004191"),
						(&NAME, "Indiana                                                                                   "),
						(&FUNCSTAT, "A"),
						(&GCUNI, "!"),
						(&POP100, "  6483802"),
						(&HU100, "  2795541"),
						(&INTPTLAT, "+39.9030256"),
						(&INTPTLON, "-086.2839503"),
						(&LSADC, "00"),
						(&PARTFLAG, " "),
						(&RESERVE3, "      "),
						(&UGA, "     "),
						(&STATENS, "00448508"),
						(&COUNTYNS, "        "),
						(&COUSUBNS, "        "),
						(&PLACENS, "        "),
						(&CONCITNS, "        "),
						(&AIANHHNS, "        "),
						(&AITSNS, "        "),
						(&ANRCNS, "        "),
						(&SUBMCDNS, "        "),
						(&CD113, "  "),
						(&CD114, "  "),
						(&CD115, "  "),
						(&SLDU2, "   "),
						(&SLDU3, "   "),
						(&SLDU4, "   "),
						(&SLDL2, "   "),
						(&SLDL3, "   "),
						(&SLDL4, "   "),
						(&AIANHHSC, "  "),
						(&CSASC, "  "),
						(&CNECTASC, "  "),
						(&MEMI, " "),
						(&NMEMI, " "),
						(&PUMA, "     "),
						(&RESERVED, "                  ")
					], fields);
				}

				#[test]
				fn example_line_fields_nonempty() {
					let fields = super::Pl94_171Layout::fields(EXAMPLE_LINE);

					assert_eq!(
						vec![
							(&FILEID, "PLST"),
							(&STUSAB, "IN"),
							(&SUMLEV, "040"),
							(&GEOCOMP, "00"),
							(&CHARITER, "000"),
							(&LOGRECNO, "0000001"),
							(&REGION, "2"),
							(&DIVISION, "3"),
							(&STATE, "18"),
							(&AREALAND, "92789193658"),
							(&AREAWATR, "1537004191"),
							(&NAME, "Indiana"),
							(&FUNCSTAT, "A"),
							(&GCUNI, "!"),
							(&POP100, "6483802"),
							(&HU100, "2795541"),
							(&INTPTLAT, "+39.9030256"),
							(&INTPTLON, "-086.2839503"),
							(&LSADC, "00"),
							(&STATENS, "00448508"),
						],
						fields
					);
				}
			}
		}
	}
}
