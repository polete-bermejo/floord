use crate::UNKNOWN;
use crate::services::service_utils::Action::Save;
use crate::services::service_utils::{rename_and_notify, rename_destination};
use anyhow::{Ok, Result};
use dirs::{audio_dir, document_dir, picture_dir, video_dir};
use std::fs::{DirEntry, create_dir_all};
use std::io;
use std::path::{Path, PathBuf};

const AUDIO_EXT: &[&str] = &[
    // Lossy
    "mp3", "aac", "ogg", "opus", "m4a", "wma", "mp2", "mp1", "ac3", "eac3", "dts", "amr", "3ga",
    "ra", "ram", "m4b", "m4p", "m4r", "mka", "spx", "voc", "vox", // Lossless
    "wav", "wave", "flac", "alac", "aiff", "aif", "aifc", "ape", "wv", "tta", "tak", "ofr", "ofs",
    "shn", "w64", "rf64", "caf", "dsf", "dff", "pcm", "raw", // MIDI / tracker
    "mid", "midi", "kar", "rmi", "mod", "xm", "it", "s3m", "mt2", "umx", "mptm", // Other
    "au", "snd", "ircam", "nist", "8svx", "cda", "pls", "m3u", "m3u8", "xspf",
];

const PICTURE_EXT: &[&str] = &[
    // Common raster
    "jpg",
    "jpeg",
    "jpe",
    "jfif",
    "png",
    "gif",
    "bmp",
    "dib",
    "webp",
    "tif",
    "tiff",
    "ico",
    "cur",
    "apng",
    "avif",
    "jxl",
    "heic",
    "heif",
    "hif",
    "bpg",
    "flif",
    // JPEG 2000
    "jp2",
    "j2k",
    "jpf",
    "jpx",
    "jpm",
    "mj2",
    "jpc",
    "j2c",
    // Camera RAW
    "raw",
    "dng",
    "cr2",
    "cr3",
    "nef",
    "nrw",
    "arw",
    "srf",
    "sr2",
    "orf",
    "rw2",
    "raf",
    "pef",
    "ptx",
    "rwl",
    "3fr",
    "fff",
    "iiq",
    "kdc",
    "dcr",
    "mos",
    "mrw",
    "x3f",
    // Editing / layered
    "psd",
    "psb",
    "xcf",
    "kra",
    "ora",
    "afphoto",
    "clip",
    "sai",
    "sai2",
    "procreate",
    // Vector
    "svg",
    "svgz",
    "eps",
    "ai",
    "cdr",
    "emf",
    "wmf",
    // Other
    "pcx",
    "tga",
    "icb",
    "vda",
    "vst",
    "dds",
    "ktx",
    "ktx2",
    "pvr",
    "exr",
    "hdr",
    "pic",
    "fits",
    "fit",
    "fts",
    "jng",
    "mng",
    "xbm",
    "xpm",
    "pbm",
    "pgm",
    "ppm",
    "pnm",
    "pam",
    "sgi",
    "rgb",
    "rgba",
    "bw",
];

const VIDEO_EXT: &[&str] = &[
    // Common
    "mp4", "m4v", "mkv", "avi", "mov", "wmv", "flv", "webm", "mpeg", "mpg", "mpe", "mpv", "3gp",
    "3g2", "ts", "mts", "m2ts", "m2t", "vob", "ogv", "ogg", // MPEG / DVD / Broadcast
    "m1v", "m2v", "m4p", "mxf", "vro", "evo", "mod", "tod", "rec", "trp", "tp", "tsp", "wtv",
    "dvr-ms", "pva", // Apple / Professional
    "qt", "f4v", "f4p", "f4a", "f4b", "prores", "r3d", "braw", // Other
    "asf", "rm", "rmvb", "ram", "divx", "xvid", "amv", "nsv", "yuv", "roq", "nuv", "nut", "y4m",
    "ivf", "mxf", "ismv", "ism", "mjpg", "mjpeg", "mkv3d",
];

const DOCUMENT_EXT: &[&str] = &[
    // Text
    "txt", "text", "md", "markdown", "mdown", "rst", "tex", "latex", "rtf", "org", "adoc",
    "asciidoc", "nfo", "log", "me", // Microsoft Office
    "doc", "docx", "docm", "dot", "dotx", "dotm", "xls", "xlsx", "xlsm", "xlsb", "xlt", "xltx",
    "ppt", "pptx", "pptm", "pps", "ppsx", "pot", "potx", "one", "vsd", "vsdx", "pub",
    // OpenDocument
    "odt", "ott", "ods", "ots", "odp", "otp", "odg", "otg", "odf", "odb",
    // PDF / ebooks
    "pdf", "epub", "mobi", "azw", "azw3", "azw4", "kfx", "djvu", "djv", "fb2", "fb3", "lit", "pdb",
    "prc", "cbz", "cbr", "cb7", "cbt", // Data / spreadsheets
    "csv", "tsv", "tab", "dif", "slk", "numbers", "ods", // Other document formats
    "pages", "key", "abw", "wpd", "wps", "wri", "sxw", "sxc", "sxi", "pages", "oxps", "xps", "ps",
    "eps",
];

/// Filter and saves file in the designated path
pub fn save(file: &DirEntry, inbox: &Path, notify: bool) -> Result<()> {
    let file_path = file.path();
    let file_name = file_path.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidFilename,
            format!("reading file_name {}", file_path.display()),
        )
    })?;


    let folder = get_destination_folder(&file_path).unwrap_or_else(|| inbox.join(UNKNOWN));
    let new_path = folder.join(file_name);

    create_dir_all(folder)?;
    
    let new_path = rename_destination(new_path);
    rename_and_notify(&file_path, &new_path, notify, &Save)?;
    Ok(())
}

fn get_destination_folder(file_path: &Path) -> Option<PathBuf> {
    let extension = file_path.extension()?.to_str()?.to_lowercase();

    let folders = [
        (AUDIO_EXT, audio_dir()),
        (PICTURE_EXT, picture_dir()),
        (VIDEO_EXT, video_dir()),
        (DOCUMENT_EXT, document_dir()),
    ];

    for (list, func) in folders {
        if list.contains(&extension.as_str()) {
            return func;
        }
    }
    None
}
