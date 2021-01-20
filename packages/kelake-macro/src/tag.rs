use crate::PeekValue;
use syn::buffer::Cursor;
struct TagStart;
impl PeekValue<()> for TagStart {
    fn peek(cursor: Cursor) -> Option<()> {
        Some(())
    }
}
