use serde::{Deserialize, Serialize};
use std::{
	borrow::Cow, cmp::Ordering, error::Error, fmt::{self, Display}, net, str::FromStr
};

use super::AmadeusOrd;

#[doc(inline)]
pub use net::{AddrParseError as ParseAddrError, IpAddr};
impl AmadeusOrd for IpAddr {
	fn amadeus_cmp(&self, other: &Self) -> Ordering {
		Ord::cmp(self, other)
	}
}

#[doc(inline)]
pub use url::{ParseError as ParseUrlError, Url};
impl AmadeusOrd for Url {
	fn amadeus_cmp(&self, other: &Self) -> Ordering {
		Ord::cmp(self, other)
	}
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub struct Webpage<'a> {
	pub ip: IpAddr,
	pub url: Url,
	pub contents: Cow<'a, [u8]>,
}
impl<'a> Webpage<'a> {
	pub fn to_owned(&self) -> Webpage<'static> {
		Webpage {
			ip: self.ip,
			url: self.url.clone(),
			contents: Cow::Owned(self.contents.clone().into_owned()),
		}
	}
}
impl<'a> AmadeusOrd for Webpage<'a> {
	fn amadeus_cmp(&self, other: &Self) -> Ordering {
		Ord::cmp(self, other)
	}
}
impl<'a> Display for Webpage<'a> {
	fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
		unimplemented!()
	}
}
impl<'a> FromStr for Webpage<'a> {
	type Err = ParseWebpageError;

	fn from_str(_s: &str) -> Result<Self, Self::Err> {
		unimplemented!()
	}
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ParseWebpageError;
impl Display for ParseWebpageError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "error parsing webpage")
	}
}
impl Error for ParseWebpageError {}
