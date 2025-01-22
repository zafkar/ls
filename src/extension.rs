// Executable Extensions
pub const EXECUTABLE_EXTENSIONS: [&str; 18] = [
    "sh",   // Shell scripts
    "bash", // Bash scripts
    "out",  // Compiled programs
    "bin",  // Binary files
    "run",  // Self-extracting installers
    "exe",  // Executable files
    "bat",  // Batch scripts
    "cmd",  // Command scripts
    "msi",  // Windows installer packages
    "com",  // Legacy command executables
    "vbs",  // VBScript files
    "ps1",  // PowerShell scripts
    "scr",  // Screensaver executables
    "jar",  // Java applications
    "py",   // Python scripts
    "pl",   // Perl scripts
    "rb",   // Ruby scripts
    "sh",   // Shell scripts
];

pub const COMPRESSED_FILE_EXTENSIONS: [&str; 16] = [
    "zip", // ZIP archive
    "rar", // RAR archive
    "7z",  // 7-Zip archive
    "tar", // Tarball (uncompressed)
    "gz",  // Gzip compressed file
    "tgz", // Gzip compressed tarball
    "bz2", // Bzip2 compressed file
    "bz",  // Bzip compressed file
    "tbz", // Bzip2 compressed tarball
    "xz",  // XZ compressed file
    "txz", // XZ compressed tarball
    "zst", // Zstandard compressed file
    "lz",  // Lzip compressed file
    "tlz", // Lzip compressed tarball
    "cab", // Microsoft Cabinet archive
    "iso", // Disk image file (may contain compressed data)
];

pub const IMAGE_FILE_EXTENSIONS: [&str; 19] = [
    "jpg",  // JPEG image
    "jpeg", // JPEG image
    "png",  // PNG image
    "gif",  // GIF image
    "bmp",  // BMP image
    "tiff", // TIFF image
    "tif",  // TIFF image
    "webp", // WebP image
    "svg",  // Scalable Vector Graphics
    "ico",  // Icon file
    "heic", // High Efficiency Image Format
    "heif", // High Efficiency Image Format
    "raw",  // Raw image format (generic)
    "cr2",  // Canon Raw image format
    "nef",  // Nikon Raw image format
    "arw",  // Sony Raw image format
    "dng",  // Digital Negative (Adobe)
    "orf",  // Olympus Raw image format
    "rw2",  // Panasonic Raw image format
];
