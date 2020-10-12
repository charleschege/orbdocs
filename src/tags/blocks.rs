pub enum BlockOps {
    ListOrdered,
    ListUnordered,
    Select,
    Paraphraph,
    Link,
    Unlink,
    Image,
    FileUpload,
    FileDownload,
    Table(TableBlocks),
    Columns,
}

pub enum TableBlocks {
    Header,
    Column,
    Row,
    Footer,
}