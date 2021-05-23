#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Parity Error Enable Register"]
    pub peen: crate::Reg<peen::PEEN_SPEC>,
    #[doc = "0x04 - Memory Checking Control Register"]
    pub mchkcon: crate::Reg<mchkcon::MCHKCON_SPEC>,
    #[doc = "0x08 - Parity Error Trap Enable Register"]
    pub pete: crate::Reg<pete::PETE_SPEC>,
    #[doc = "0x0c - Parity Error Reset Enable Register"]
    pub persten: crate::Reg<persten::PERSTEN_SPEC>,
    _reserved4: [u8; 4usize],
    #[doc = "0x14 - Parity Error Flag Register"]
    pub peflag: crate::Reg<peflag::PEFLAG_SPEC>,
    #[doc = "0x18 - Parity Memory Test Pattern Register"]
    pub pmtpr: crate::Reg<pmtpr::PMTPR_SPEC>,
    #[doc = "0x1c - Parity Memory Test Select Register"]
    pub pmtsr: crate::Reg<pmtsr::PMTSR_SPEC>,
}
#[doc = "PEEN register accessor: an alias for `Reg<PEEN_SPEC>`"]
pub type PEEN = crate::Reg<peen::PEEN_SPEC>;
#[doc = "Parity Error Enable Register"]
pub mod peen;
#[doc = "MCHKCON register accessor: an alias for `Reg<MCHKCON_SPEC>`"]
pub type MCHKCON = crate::Reg<mchkcon::MCHKCON_SPEC>;
#[doc = "Memory Checking Control Register"]
pub mod mchkcon;
#[doc = "PETE register accessor: an alias for `Reg<PETE_SPEC>`"]
pub type PETE = crate::Reg<pete::PETE_SPEC>;
#[doc = "Parity Error Trap Enable Register"]
pub mod pete;
#[doc = "PERSTEN register accessor: an alias for `Reg<PERSTEN_SPEC>`"]
pub type PERSTEN = crate::Reg<persten::PERSTEN_SPEC>;
#[doc = "Parity Error Reset Enable Register"]
pub mod persten;
#[doc = "PEFLAG register accessor: an alias for `Reg<PEFLAG_SPEC>`"]
pub type PEFLAG = crate::Reg<peflag::PEFLAG_SPEC>;
#[doc = "Parity Error Flag Register"]
pub mod peflag;
#[doc = "PMTPR register accessor: an alias for `Reg<PMTPR_SPEC>`"]
pub type PMTPR = crate::Reg<pmtpr::PMTPR_SPEC>;
#[doc = "Parity Memory Test Pattern Register"]
pub mod pmtpr;
#[doc = "PMTSR register accessor: an alias for `Reg<PMTSR_SPEC>`"]
pub type PMTSR = crate::Reg<pmtsr::PMTSR_SPEC>;
#[doc = "Parity Memory Test Select Register"]
pub mod pmtsr;
