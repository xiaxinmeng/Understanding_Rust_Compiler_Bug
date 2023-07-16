rust
let current = cursor.seek(SeekFrom::Current(0))?;
let end = cursor.seek(SeekFrom::End(0))?;
if (current != end) {
    cursor.seek(SeekFrom::Start(current))?;
}
