#[cfg(feature = "pcg64")]
pub use rand_pcg::{ Lcg64Xsh32 };

#[cfg(feature = "pcg128")]
pub use rand_pcg::{ Mcg128Xsl64 };

