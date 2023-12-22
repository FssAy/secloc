//! SecLoc - A Rust library for locating and working with sections in the current executable.

mod core;

use core::*;

/// Represents a PE section.
///
/// Use `secloc::SecLoc` to retrieve sections.
pub struct Section {
    /// The number of the section with the retained order.
    pub num: usize,
    pub name: &'static str,
    /// Memory address where the section starts.
    pub virtual_address: usize,
    /// Size of the section's data.
    pub data_size: usize,
}

/// Provides functionality to locate and interact with PE sections in the current executable.
pub struct SecLoc {
    module_handle: HMODULE,
    nt_headers: *const IMAGE_NT_HEADERS,
}

impl SecLoc {
    pub unsafe fn new() -> Self {
        let module_handle = get_base_module_handle();
        let nt_headers = get_nt_headers(module_handle);

        Self {
            module_handle,
            nt_headers,
        }
    }

    /// Retrieves all the sections.
    ///
    /// # Returns
    /// A vector of `Section` objects.
    pub unsafe fn get_all(&self) -> Vec<Section> {
        let mut sections = Vec::with_capacity(
            self.get_sections_amount() as usize
        );

        self.on_pe_sections(|section| {
            sections.push(section);
            true
        });

        sections
    }

    /// Finds a specific section by name.
    ///
    /// # Arguments
    /// * `section_name` - The name of the section to find.
    ///
    /// # Returns
    /// An `Option<Section>` which is `Some` if the section is found, otherwise `None`.
    pub unsafe fn find(&self, section_name: impl AsRef<str>) -> Option<Section> {
        let name = section_name.as_ref();
        let mut section_out = None;

        self.on_pe_sections(|section| {
            let found = section.name == name;

            if found {
                section_out = Some(section);
            }

            !found
        });

        section_out
    }

    /// Iterates over all PE sections and applies a callback function to each.
    ///
    /// # Arguments
    /// * `callback` - A mutable callback function applied to each `Section`.
    /// When the callback returns false it stops from getting the next sections.
    ///
    /// # Example
    /// ```rust
    ///  SecLoc::new()
    ///     .on_pe_sections(|section| {
    ///         println!(
    ///             "Section {} [{}] at [0x{:X}] of size [{}]",
    ///             section.num,
    ///             section.name,
    ///             section.virtual_address,
    ///             section.data_size,
    ///         );
    ///         true
    ///  });
    /// ```
    pub unsafe fn on_pe_sections<F: FnMut(Section) -> bool>(&self, mut callback: F) {
        let section_headers_start = (self.nt_headers as usize + std::mem::size_of::<IMAGE_NT_HEADERS>()) as *const IMAGE_SECTION_HEADER;
        let number_of_sections = self.get_sections_amount();
        let base_address = self.module_handle as usize;

        for i in 0..number_of_sections as usize {
            let section_header = section_headers_start.offset(i as isize);

            let section_name = std::slice::from_raw_parts((*section_header).Name.as_ptr(), 8);
            let name = trim_null_bytes(std::str::from_utf8_unchecked(section_name));

            let section = Section {
                num: i,
                name,
                virtual_address: base_address + (*section_header).VirtualAddress as usize,
                data_size: (*section_header).SizeOfRawData as usize,
            };

            if !callback(section) {
                break;
            }
        }
    }

    /// Retrieves the amount of sections in the PE.
    ///
    /// # Returns
    /// The number of sections in the PE as `u16`.
    unsafe fn get_sections_amount(&self) -> u16 {
        (*(self.nt_headers)).FileHeader.NumberOfSections
    }
}
