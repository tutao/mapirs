pub use file_descriptor::{FileDescriptor, FileTagExtension, RawMapiFileDesc, RawMapiFileTagExt};
pub use message::{Message, RawMapiMessage};
pub use recipient_descriptor::{RawMapiRecipDesc, RecipientDescriptor};

mod file_descriptor;
mod message;
mod recipient_descriptor;
