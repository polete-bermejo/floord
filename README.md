# Floor daemon
**Floord** is a lightweight background daemon written in Rust that automatically manages and organizes incoming files.

## File Organization

**Floord** automatically organizes incoming files based on their file extensions, sorting them into the appropriate system directories according to their file type.

It uses the operating system's standard directories whenever available:

- **Documents**: .pdf, .docx, .txt, .odt, etc.
- **Music**: .mp3, .wav, .flac, .ogg, etc.
- **Videos**: .mp4, .mkv, .avi, .mov, etc.
- **Pictures**: .jpg, .jpeg, .png, .gif, .webp, etc.

**Unknown files**: If a file has an unsupported or unrecognized extension, Floord automatically creates an Unknown folder inside the designated organization directory (if it doesn't already exist) and moves the file there.

## Installation
Execute:
```bash
curl -fsSL https://raw.githubusercontent.com/polete-bermejo/floord/main/install.sh | sh
```

## Service Guide
To see the available commands and options, run:
```bash
floord --help
```
