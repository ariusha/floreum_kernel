use limine::{
    BaseRevision,
    request::{HhdmRequest, MemmapRequest, PagingModeRequest, RsdpRequest},
};

#[used]
#[unsafe(link_section = ".requests")]
pub static RSDP: RsdpRequest = RsdpRequest::new();
#[used]
#[unsafe(link_section = ".requests")]
pub static PAGING_MODE: PagingModeRequest = PagingModeRequest::PREFER_MAXIMUM;
