use core::ops::Range;

pub const FILEID: Range<usize> = 0..6;
pub const STUSAB: Range<usize> = 6..8;
pub const SUMLEV: Range<usize> = 8..11;
pub const GEOCOMP: Range<usize> = 11..13;
pub const CHARITER: Range<usize> = 13..16;
pub const CIFSN: Range<usize> = 16..18;
pub const LOGRECNO: Range<usize> = 18..25;
pub const REGION: Range<usize> = 25..26;
pub const DIVISION: Range<usize> = 26..27;
pub const STATE: Range<usize> = 27..29;
pub const COUNTY: Range<usize> = 29..32;
pub const COUNTYCC: Range<usize> = 32..34;
pub const COUNTYSC: Range<usize> = 34..36;
pub const COUSUB: Range<usize> = 36..41;
pub const COUSUBCC: Range<usize> = 41..43;
pub const COUSUBSC: Range<usize> = 43..45;
pub const PLACE: Range<usize> = 45..50;
pub const PLACECC: Range<usize> = 50..52;
pub const PLACESC: Range<usize> = 52..54;
pub const TRACT: Range<usize> = 54..60;
pub const BLKGRP: Range<usize> = 60..61;
pub const BLOCK: Range<usize> = 61..65;
pub const IUC: Range<usize> = 65..67;
pub const CONCIT: Range<usize> = 67..72;
pub const CONCITCC: Range<usize> = 72..74;
pub const CONCITSC: Range<usize> = 74..76;
pub const AIANHH: Range<usize> = 76..80;
pub const AIANHHFP: Range<usize> = 80..85;
pub const AIANHHCC: Range<usize> = 85..87;
pub const AIHHTLI: Range<usize> = 87..88;
pub const AITSCE: Range<usize> = 88..91;
pub const AITS: Range<usize> = 91..96;
pub const AITSCC: Range<usize> = 96..98;
pub const TTRACT: Range<usize> = 98..104;
pub const TBLKGRP: Range<usize> = 104..105;
pub const ANRC: Range<usize> = 105..110;
pub const ANRCCC: Range<usize> = 110..112;
pub const CBSA: Range<usize> = 112..117;
pub const CBASC: Range<usize> = 117..119;
pub const METDIV: Range<usize> = 119..124;
pub const CSA: Range<usize> = 124..127;
pub const NECTA: Range<usize> = 127..132;
pub const NECTASC: Range<usize> = 132..134;
pub const NECTADIV: Range<usize> = 134..139;
pub const CNECTA: Range<usize> = 139..142;
pub const CBSAPCI: Range<usize> = 142..143;
pub const NECTAPCI: Range<usize> = 143..144;
pub const UA: Range<usize> = 144..149;
pub const UASC: Range<usize> = 149..151;
pub const UATYPE: Range<usize> = 151..152;
pub const UR: Range<usize> = 152..153;
pub const CD: Range<usize> = 153..155;
pub const SLDU: Range<usize> = 155..158;
pub const SLDL: Range<usize> = 158..161;
pub const VTD: Range<usize> = 161..167;
pub const VTDI: Range<usize> = 167..168;
pub const RESERVE2: Range<usize> = 168..171;
pub const ZCTA5: Range<usize> = 171..176;
pub const SUBMCD: Range<usize> = 176..181;
pub const SUBMCDCC: Range<usize> = 181..183;
pub const SDELM: Range<usize> = 183..188;
pub const SDSEC: Range<usize> = 188..193;
pub const SDUNI: Range<usize> = 193..198;
pub const AREALAND: Range<usize> = 198..212;
pub const AREAWATR: Range<usize> = 212..226;
pub const NAME: Range<usize> = 226..316;
pub const FUNCSTAT: Range<usize> = 316..317;
pub const GCUNI: Range<usize> = 317..318;
pub const POP100: Range<usize> = 318..327;
pub const HU100: Range<usize> = 327..336;
pub const INTPTLAT: Range<usize> = 336..347;
pub const INTPTLON: Range<usize> = 347..359;
pub const LSADC: Range<usize> = 359..361;
pub const PARTFLAG: Range<usize> = 361..362;
pub const RESERVE3: Range<usize> = 362..368;
pub const UGA: Range<usize> = 368..373;
pub const STATENS: Range<usize> = 373..381;
pub const COUNTYNS: Range<usize> = 381..389;
pub const COUSUBNS: Range<usize> = 389..397;
pub const PLACENS: Range<usize> = 397..405;
pub const CONCITNS: Range<usize> = 405..413;
pub const AIANHHNS: Range<usize> = 413..421;
pub const AITSNS: Range<usize> = 421..429;
pub const ANRCNS: Range<usize> = 429..437;
pub const SUBMCDNS: Range<usize> = 437..445;
pub const CD113: Range<usize> = 445..447;
pub const CD114: Range<usize> = 447..449;
pub const CD115: Range<usize> = 449..451;
pub const SLDU2: Range<usize> = 451..454;
pub const SLDU3: Range<usize> = 454..457;
pub const SLDU4: Range<usize> = 457..460;
pub const SLDL2: Range<usize> = 460..463;
pub const SLDL3: Range<usize> = 463..466;
pub const SLDL4: Range<usize> = 466..469;
pub const AIANHHSC: Range<usize> = 469..471;
pub const CSASC: Range<usize> = 471..473;
pub const CNECTASC: Range<usize> = 473..475;
pub const MEMI: Range<usize> = 475..476;
pub const NMEMI: Range<usize> = 476..477;
pub const PUMA: Range<usize> = 477..482;
pub const RESERVED: Range<usize> = 482..500;

#[cfg(test)]
macro_rules! verify_range {
	($name:ident, $a:literal..$b:literal) => {
		#[cfg(test)]
		#[allow(non_snake_case)]
		mod $name {
			use super::*;

			#[test]
			fn is_correct() {
				assert_eq!($name, $a..$b);
			}
		}
	};
}

#[cfg(test)]
mod tests {
	use super::*;

	mod correctness {
		use super::*;

		verify_range!(FILEID, 0..6);
		verify_range!(STUSAB, 6..8);
		verify_range!(SUMLEV, 8..11);
		verify_range!(GEOCOMP, 11..13);
		verify_range!(CHARITER, 13..16);
		verify_range!(CIFSN, 16..18);
		verify_range!(LOGRECNO, 18..25);
		verify_range!(REGION, 25..26);
		verify_range!(DIVISION, 26..27);
		verify_range!(STATE, 27..29);
		verify_range!(COUNTY, 29..32);
		verify_range!(COUNTYCC, 32..34);
		verify_range!(COUNTYSC, 34..36);
		verify_range!(COUSUB, 36..41);
		verify_range!(COUSUBCC, 41..43);
		verify_range!(COUSUBSC, 43..45);
		verify_range!(PLACE, 45..50);
		verify_range!(PLACECC, 50..52);
		verify_range!(PLACESC, 52..54);
		verify_range!(TRACT, 54..60);
		verify_range!(BLKGRP, 60..61);
		verify_range!(BLOCK, 61..65);
		verify_range!(IUC, 65..67);
		verify_range!(CONCIT, 67..72);
		verify_range!(CONCITCC, 72..74);
		verify_range!(CONCITSC, 74..76);
		verify_range!(AIANHH, 76..80);
		verify_range!(AIANHHFP, 80..85);
		verify_range!(AIANHHCC, 85..87);
		verify_range!(AIHHTLI, 87..88);
		verify_range!(AITSCE, 88..91);
		verify_range!(AITS, 91..96);
		verify_range!(AITSCC, 96..98);
		verify_range!(TTRACT, 98..104);
		verify_range!(TBLKGRP, 104..105);
		verify_range!(ANRC, 105..110);
		verify_range!(ANRCCC, 110..112);
		verify_range!(CBSA, 112..117);
		verify_range!(CBASC, 117..119);
		verify_range!(METDIV, 119..124);
		verify_range!(CSA, 124..127);
		verify_range!(NECTA, 127..132);
		verify_range!(NECTASC, 132..134);
		verify_range!(NECTADIV, 134..139);
		verify_range!(CNECTA, 139..142);
		verify_range!(CBSAPCI, 142..143);
		verify_range!(NECTAPCI, 143..144);
		verify_range!(UA, 144..149);
		verify_range!(UASC, 149..151);
		verify_range!(UATYPE, 151..152);
		verify_range!(UR, 152..153);
		verify_range!(CD, 153..155);
		verify_range!(SLDU, 155..158);
		verify_range!(SLDL, 158..161);
		verify_range!(VTD, 161..167);
		verify_range!(VTDI, 167..168);
		verify_range!(RESERVE2, 168..171);
		verify_range!(ZCTA5, 171..176);
		verify_range!(SUBMCD, 176..181);
		verify_range!(SUBMCDCC, 181..183);
		verify_range!(SDELM, 183..188);
		verify_range!(SDSEC, 188..193);
		verify_range!(SDUNI, 193..198);
		verify_range!(AREALAND, 198..212);
		verify_range!(AREAWATR, 212..226);
		verify_range!(NAME, 226..316);
		verify_range!(FUNCSTAT, 316..317);
		verify_range!(GCUNI, 317..318);
		verify_range!(POP100, 318..327);
		verify_range!(HU100, 327..336);
		verify_range!(INTPTLAT, 336..347);
		verify_range!(INTPTLON, 347..359);
		verify_range!(LSADC, 359..361);
		verify_range!(PARTFLAG, 361..362);
		verify_range!(RESERVE3, 362..368);
		verify_range!(UGA, 368..373);
		verify_range!(STATENS, 373..381);
		verify_range!(COUNTYNS, 381..389);
		verify_range!(COUSUBNS, 389..397);
		verify_range!(PLACENS, 397..405);
		verify_range!(CONCITNS, 405..413);
		verify_range!(AIANHHNS, 413..421);
		verify_range!(AITSNS, 421..429);
		verify_range!(ANRCNS, 429..437);
		verify_range!(SUBMCDNS, 437..445);
		verify_range!(CD113, 445..447);
		verify_range!(CD114, 447..449);
		verify_range!(CD115, 449..451);
		verify_range!(SLDU2, 451..454);
		verify_range!(SLDU3, 454..457);
		verify_range!(SLDU4, 457..460);
		verify_range!(SLDL2, 460..463);
		verify_range!(SLDL3, 463..466);
		verify_range!(SLDL4, 466..469);
		verify_range!(AIANHHSC, 469..471);
		verify_range!(CSASC, 471..473);
		verify_range!(CNECTASC, 473..475);
		verify_range!(MEMI, 475..476);
		verify_range!(NMEMI, 476..477);
		verify_range!(PUMA, 477..482);
		verify_range!(RESERVED, 482..500);
	}
}
