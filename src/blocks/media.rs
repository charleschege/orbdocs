enum ImageType {
    PNG,
    JPEG,
    WebP,
    GIF,
    AVIF,
}

struct Image {
    url: String, //FIXME to URL
    ext: ImageType,
}

struct Favicon;
