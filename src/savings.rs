use dirs::{audio_dir, document_dir, picture_dir, video_dir};
use std::fs::{DirEntry, create_dir_all, rename};
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
pub fn save(file: DirEntry, inbox: &Path) {
    let file_path = file.path();
    let Some(file_name) = file_path.file_name() else {
        return;
    };
    let mut new_path = inbox.join("Unknown").join(file_name);

    if file_path.is_dir() && file_name == "Unknown" {
        return;
    }

    if let Some(folder) = get_destination_folder(&file_path) {
        new_path = folder.join(file_name);
        rename_destination(&mut new_path);
        let _ = rename(file_path, new_path);
        return;
    }
    if let Some(unknown_path) = new_path.parent()
        && !unknown_path.exists()
    {
        create_dir_all(unknown_path).unwrap_or(())
    }
    rename_destination(&mut new_path);
    let _ = rename(file_path, new_path);
}

fn rename_destination(new_path: &mut PathBuf) {
    if let (Some(file_name), Some(folder)) = (new_path.file_stem(), new_path.parent()) {
        let file_name = file_name.to_string_lossy().to_string();
        let folder = folder.to_path_buf();

        let extension = new_path.extension().map(|e| e.to_os_string());

        let (base_name, mut count) = match file_name.rsplit_once('(') {
            Some((name, number)) => {
                match number.strip_suffix(')').and_then(|n| n.parse::<u32>().ok()) {
                    Some(n) if !name.is_empty() => (name.to_string(), n + 1),
                    _ => (file_name.clone(), 1),
                }
            }
            None => (file_name.clone(), 1),
        };

        while new_path.exists() {
            let new_name = match &extension {
                Some(ext) => format!("{} ({}).{}", base_name, count, ext.to_string_lossy()),
                None => format!("{}({})", base_name, count),
            };

            *new_path = folder.join(new_name);
            count += 1;
        }
    }
}

fn get_destination_folder(file_path: &Path) -> Option<PathBuf> {
    if let Some(extension) = file_path.extension().and_then(|i| i.to_str()) {
        let folders = [
            (AUDIO_EXT, audio_dir()),
            (PICTURE_EXT, picture_dir()),
            (VIDEO_EXT, video_dir()),
            (DOCUMENT_EXT, document_dir()),
        ];

        for (list, func) in folders {
            if list.contains(&extension) {
                return func;
            }
        }
    }
    None
}
