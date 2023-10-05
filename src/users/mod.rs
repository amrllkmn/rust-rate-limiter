struct User<'a> {
    id: u32,
    bucket: &'a bucket::Bucket,
}

mod bucket {
    pub struct Bucket {}
}
