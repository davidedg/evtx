use crate::binxml::tokens::read_template_definition;

use crate::model::deserialized::BinXMLTemplateDefinition;
use crate::Offset;
pub use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::io::{Cursor, Seek, SeekFrom};
use std::rc::Rc;
use crate::evtx_chunk::EvtxChunk;

pub type CachedTemplate<'c> = BinXMLTemplateDefinition<'c>;

#[derive(Debug, Default)]
pub struct TemplateCache<'c>(HashMap<Offset, CachedTemplate<'c>>);

impl<'c> TemplateCache<'c> {
    pub fn new() -> Self {
        TemplateCache(HashMap::new())
    }

    pub fn populate(data: &'c [u8], offsets: &[Offset]) -> Result<Self, failure::Error> {
        let mut cache = TemplateCache(HashMap::new());
        let mut cursor = Cursor::new(data);

        for offset in offsets.iter().filter(|&&offset| offset > 0) {
            cursor.seek(SeekFrom::Start(u64::from(*offset)))?;
            let definition = read_template_definition(&mut cursor, None)?;
            cache.0.insert(*offset, definition);
        }

        Ok(cache)
    }

    pub fn get_template<'a: 'c>(&'a self, offset: Offset) -> Option<&'a CachedTemplate<'a>> {
        self.0.get(&offset).into()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
