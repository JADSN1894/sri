use std::fs;

use crate::error::SriResult;

pub(super) fn replace_content_in_file(
  file_path: &str,
  target: &str,
  replacement: &str,
) -> SriResult<()> {
  // Step 1: Read the file content
  let mut content = fs::read_to_string(file_path)?;

  // Step 2: Replace the content
  content = content.replace(target, replacement);

  // Step 3: Write the modified content back to the file
  fs::write(file_path, &content)?;

  Ok(())
}
