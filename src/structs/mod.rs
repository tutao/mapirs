pub use file_descriptor::{FileDescriptor, FileTagExtension, RawMapiFileDesc, RawMapiFileTagExt};
pub use message::{Message, RawMapiMessage};
pub use recipient_descriptor::{RawMapiRecipDesc, RecipientDescriptor};

mod message;
mod recipient_descriptor;
mod file_descriptor;