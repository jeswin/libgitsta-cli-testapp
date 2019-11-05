pub struct CollectionInfo<'a> {
    collection_type: &'a str
}


pub struct QueryParam {

}

pub struct QueryInfo<'a> {
    name: &'a str,
    sql: &'a str,
    params: Vec<QueryParam>
}

pub struct Configuration<'a> {
    collections: Vec<CollectionInfo<'a>>
}

pub fn init(config: Configuration) {
    
}